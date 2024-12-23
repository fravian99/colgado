use crate::word::Game;
use std::cell::OnceCell;

use colgado_requests::models::info;
use colgado_requests::requests;
use tokio::sync::{mpsc, oneshot};

use super::models::game_view::GameView;
use super::models::messages::{CommandMessage, GeneralMessage, TwitchMessage};

pub struct TwitchGameActor {
    receiver: mpsc::UnboundedReceiver<GeneralMessage>,
    session_id: OnceCell<String>,
    game: Option<Game>,
    user_info: info::User,
    bot_info: info::Bot,
    command: String,
}

impl TwitchGameActor {
    pub fn new(
        receiver: mpsc::UnboundedReceiver<GeneralMessage>,
        user_info: info::User,
        bot_info: info::Bot,
        command: String,
    ) -> Self {
        let session_id = OnceCell::<String>::new();
        let command = command + " ";
        Self {
            receiver,
            session_id,
            game: None,
            user_info,
            bot_info,
            command,
        }
    }

    async fn handle(&mut self, message: GeneralMessage) -> bool {
        match message {
            GeneralMessage::CommandMessage(command) => self.handle_command_message(command),
            GeneralMessage::TwitchMessage(message) => self.handle_twitch_message(message).await,
            GeneralMessage::TwitchSendMessage(message) => {
                let _ = requests::send_msg_request(
                    &self.bot_info,
                    &self.user_info.user_id,
                    &self.user_info.user_id,
                    &message,
                )
                .await;
                true
            }
        }
    }

    async fn handle_twitch_message(&mut self, message: TwitchMessage) -> bool {
        match message {
            TwitchMessage::WelcomeMessage { session_id } => {
                colgado_requests::subscribe_to_wb(
                    &self.bot_info,
                    &session_id,
                    &self.user_info.user_id,
                    &self.user_info.user_id,
                )
                .await
                .expect("Error suscribing to channel");
                self.session_id.set(session_id).expect("Error setting id")
            }
            TwitchMessage::PlayerMessage {
                message_text,
                message_id,
                player_id: _,
                player_name: _,
            } => {
                if self.game.is_some() {
                    let message_text: &str = &message_text;
                    let word_chars = self.valid_player_message(message_text);
                    if let (Some(word_chars), Some(game)) = (word_chars, &mut self.game) {
                        let result = game.check_word_chars(&word_chars);
                        if let Err(err) = result {
                            let _ = requests::send_msg_reply_request(
                                &self.bot_info,
                                &self.user_info.user_id,
                                &self.user_info.user_id,
                                &message_id,
                                err.twitch_message_error(),
                            )
                            .await;
                        }
                    }
                }
            }

            _ => {}
        }
        true
    }

    fn valid_player_message<'a>(&self, mut message_text: &'a str) -> Option<Vec<&'a str>> {
        if message_text.is_empty() {
            return None;
        }
        // command includes a space
        let command_included = message_text.len() > self.command.len()
            && message_text[..self.command.len()] == self.command;
        if command_included {
            message_text = &message_text[self.command.len()..];
        }
        let word_chars: Vec<&str> = Game::split_chars(message_text);
        if command_included || word_chars.len() == 1 {
            Some(word_chars)
        } else {
            None
        }
    }

    fn handle_command_message(&mut self, message: CommandMessage) -> bool {
        match message {
            CommandMessage::GetSessionId { sender } => {
                let id = self.session_id.get().cloned();
                let _ = sender.send(id);
            }
            CommandMessage::GetGameState { sender } => {
                let game_view = match &self.game {
                    Some(game) => {
                        let game_view = GameView::from(game);
                        Some(game_view)
                    }
                    None => None,
                };
                let _ = sender.send(game_view);
            }
            CommandMessage::SetGameWord { word, sender } => {
                let game = Game::new(word);
                let word = game.get_actual_word();
                self.game = Some(game);
                let _ = sender.send(word);
            }
        }
        true
    }

    pub async fn run(mut self) {
        while let Some(message) = self.receiver.recv().await {
            let continue_loop = self.handle(message).await;
            if !continue_loop {
                break;
            }
        }
        println!("[actor TwitchGameActor]: Finished");
    }
}

#[derive(Debug, Clone)]
pub struct TwitchGameHandle {
    sender: mpsc::UnboundedSender<GeneralMessage>,
}

impl TwitchGameHandle {
    pub fn new_and_joinhandle(
        user_info: info::User,
        bot_info: info::Bot,
        command: String,
    ) -> (Self, tokio::task::JoinHandle<()>) {
        let (send, recv) = mpsc::unbounded_channel();
        let actor = TwitchGameActor::new(recv, user_info, bot_info, command);
        let task = tokio::spawn(actor.run());
        (Self { sender: send }, task)
    }

    async fn send_and_recv<MSType>(
        &self,
        message: CommandMessage,
        recv: oneshot::Receiver<MSType>,
    ) -> MSType {
        let message = GeneralMessage::CommandMessage(message);
        self.sender
            .send(message)
            .expect("Error: Twitch Actor killed");
        recv.await.expect("Error: Twitch Actor killed")
    }

    pub async fn get_id(&self) -> Option<String> {
        let (send, recv) = oneshot::channel::<Option<String>>();
        let message = CommandMessage::GetSessionId { sender: send };
        self.send_and_recv(message, recv).await
    }

    pub async fn get_game_state(&self) -> Option<GameView> {
        let (send, recv) = oneshot::channel::<Option<GameView>>();
        let message = CommandMessage::GetGameState { sender: send };
        self.send_and_recv(message, recv).await
    }

    pub async fn set_game_word(&self, word: String) -> String {
        let (send, recv) = oneshot::channel::<String>();
        let message = CommandMessage::SetGameWord { word, sender: send };
        self.send_and_recv(message, recv).await
    }

    pub fn non_sleeping_send(&self, message: GeneralMessage) {
        // since it is an unbound channel it can not sleep,
        // in case of bounded channels try_send is needed
        let _ = self.sender.send(message);
    }

    pub async fn send_message(&self, message: String) {
        let message = GeneralMessage::TwitchSendMessage(message);
        let _ = self.sender.send(message);
    }

    pub async fn send_messages<IntoStringIter>(&self, messages: IntoStringIter)
    where
        IntoStringIter: IntoIterator<Item = String>,
    {
        for message in messages {
            self.send_message(message).await;
        }
    }
}
