#![windows_subsystem = "windows"]
use std::sync::Arc;

use colgado_gui::init_flow;
use colgado_logic::{
    errors::ColgadoLogicError,
    models::{game_view::GameView, handles::Handles},
};
use iced::{
    widget::{self, button, center, column, row, text, text_input, Column},
    window::{self, close_requests},
    Alignment::Center,
    Element, Font,
    Length::Fill,
    Subscription, Task,
};
pub type ClonableResult<T, E> = Result<T, Arc<E>>;
pub type LogicResult<T> = ClonableResult<T, ColgadoLogicError>;

pub const FONT: &[u8] =
    include_bytes!("../assets/fonts/RobotoMonoNerdFontMono-Regular.ttf").as_slice();
pub const TEXT: Font = Font::with_name("RobotoMono Nerd Font Mono");

pub const ICON: &[u8] = include_bytes!("../assets/logo.png");

fn main() -> iced::Result {
    let application = iced::application(
        "El que tengo aquí colgado",
        ColgadoApp::update,
        ColgadoApp::view,
    )
    .window(window::Settings {
        icon: Some(
            window::icon::from_file_data(include_bytes!("../assets/logo.png"), None).unwrap(),
        ),
        ..Default::default()
    })
    .default_font(TEXT)
    .font(FONT)
    .subscription(ColgadoApp::subscription)
    .antialiasing(true)
    .centered()
    .exit_on_close_request(false);
    application.run()
}

#[derive(Clone, Debug)]
pub enum Message {
    NewConnection,
    Connected(LogicResult<(Handles, Arc<[tokio::task::JoinHandle<()>]>)>),
    NewGame,
    NewWord(String),
    WordSetted(String),
    SubmitWord,
    GetActualState,
    ActualState(Option<GameView>),
    Close(window::Id),
}
#[derive(Clone, Debug)]
pub enum State {
    NewConnection,
    Connecting,
    NewWord,
    SettingGame,
    Playing,
    GameCompleted,
}
#[derive(Clone, Debug)]
pub struct ColgadoApp {
    title: &'static str,
    game: GameView,
    state: State,
    tasks: Option<Arc<[tokio::task::JoinHandle<()>]>>,
    handles: Option<Handles>,
}

impl ColgadoApp {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NewConnection => {
                if let State::NewConnection = self.state {
                    self.state = State::Connecting;
                    return self.connect();
                }
            }
            Message::Connected(result) => {
                let ok_value = match result {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("{}", err);
                        self.state = State::NewConnection;
                        return Task::none();
                    }
                };
                let (handles, tasks) = ok_value;
                self.state = State::NewWord;
                self.handles = Some(handles);
                self.tasks = Some(tasks);
            }
            Message::NewGame => {
                self.state = State::NewWord;
                self.game = GameView::default();
            }
            Message::NewWord(word) => {
                self.game.word = word;
            }
            Message::SubmitWord => {
                if !self.game.word.is_empty() {
                    self.state = State::SettingGame;
                    return self.send_new_word();
                }
            }
            Message::WordSetted(word) => {
                self.game.word = word;
                self.game.is_completed = false;
                self.state = State::Playing;
                return self.send_message("Comenzando partida".to_string());
            }
            Message::GetActualState => {
                if !self.game.is_completed {
                    return self.get_game();
                }
                if let State::Playing = self.state {
                    self.state = State::GameCompleted;
                    return self.send_messages(vec![
                        "Partida terminada".to_string(),
                        format!("La palabra era {}", self.game.word),
                    ]);
                }
            }
            Message::ActualState(Some(game)) => {
                self.game = game;
            }
            Message::Close(id) => {
                if let Some(tasks) = &self.tasks {
                    tasks.iter().for_each(|task| {
                        if !task.is_finished() {
                            task.abort();
                        }
                    });
                    println!("all aborted");
                }
                return window::close::<Message>(id);
            }
            _ => {}
        }
        Task::none()
    }

    fn connect(&self) -> Task<Message> {
        Task::perform(init_flow(), Message::Connected)
    }

    fn send_new_word(&self) -> Task<Message> {
        if let Some(handles) = &self.handles {
            let game_handle = handles.game_handle.clone();
            let word = self.game.word.clone();
            return Task::perform(
                async move { game_handle.set_game_word(word).await },
                Message::WordSetted,
            );
        }
        Task::none()
    }

    fn send_message(&self, word: String) -> Task<Message> {
        if let Some(handles) = &self.handles {
            let game_handle = handles.game_handle.clone();
            return Task::perform(async move { game_handle.send_message(word).await }, |_| {
                Message::GetActualState
            });
        }
        Task::none()
    }

    fn send_messages(&self, words: Vec<String>) -> Task<Message> {
        if let Some(handles) = &self.handles {
            let game_handle = handles.game_handle.clone();
            return Task::perform(
                async move { game_handle.send_messages(words).await },
                |_| Message::GetActualState,
            );
        }
        Task::none()
    }

    fn get_game(&self) -> Task<Message> {
        if let Some(handles) = &self.handles {
            let game_handle = handles.game_handle.clone();
            return Task::perform(
                async move { game_handle.get_game_state().await },
                Message::ActualState,
            );
        }
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        let title = text(self.title).font(TEXT).size(30);
        let title = row![title];
        let mut view = match self.state {
            State::NewConnection | State::Connecting => self.new_connection_view(),
            State::NewWord | State::SettingGame => self.new_word_view(),
            State::Playing | State::GameCompleted => self.playing_view(),
        };

        view = view.max_width(600);
        view = column![title, view].spacing(40).align_x(Center).width(Fill);
        let view = center(view);
        widget::container(view).into()
    }

    fn new_connection_view(&self) -> Column<Message> {
        let mut button = button(text("Conectar"));
        button = if let State::NewConnection = self.state {
            button.on_press(Message::NewConnection)
        } else {
            button
        };
        column![button].width(Fill).align_x(Center)
    }

    fn new_word_view(&self) -> Column<Message> {
        let mut send_button = button(text("Jugar"));
        send_button = if let State::NewWord = self.state {
            send_button.on_press(Message::SubmitWord)
        } else {
            send_button
        };
        let word_input = column![
            text("Introduce una palabra:"),
            row![
                text_input("Palabra", &self.game.word).on_input(Message::NewWord),
                send_button,
            ]
        ];
        column![word_input].width(Fill).align_x(Center)
    }

    fn playing_view(&self) -> Column<Message> {
        let text_input = text(&self.game.word);
        let word_input = column![text_input.size(40)];
        let letters = column![text(&self.game.letters).size(40)];
        let mut column = column![word_input, letters].spacing(10);

        if let State::GameCompleted = &self.state {
            let button = button(text("Nueva partida")).on_press(Message::NewGame);
            column = column.push(button);
        }
        column.width(Fill).align_x(Center)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions = Vec::with_capacity(2);
        let close_event: Subscription<Message> =
            close_requests().map(|id: window::Id| Message::Close(id));
        subscriptions.push(close_event);
        if let State::Playing = self.state {
            let game_subscription = iced::time::every(iced::time::Duration::from_millis(10))
                .map(|_| Message::GetActualState);
            subscriptions.push(game_subscription);
        }

        Subscription::batch(subscriptions)
    }
}

impl Default for ColgadoApp {
    fn default() -> Self {
        Self {
            title: "El que tengo aquí colgado",
            game: GameView::default(),
            state: State::NewConnection,
            tasks: None,
            handles: None,
        }
    }
}
