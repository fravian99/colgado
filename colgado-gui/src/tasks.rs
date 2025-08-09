use std::{future::Future, sync::Arc};

use iced::Task;
use iced_futures::MaybeSend;

pub struct TaskBuider<C, M> {
    closure: Option<C>,
    mapping: Option<M>,
}

impl Default for TaskBuider<(), ()> {
    fn default() -> Self {
        Self {
            closure: None,
            mapping: None,
        }
    }
}

impl<C, M> TaskBuider<C, M> {
    pub fn set_closure<S>(self, closure: S) -> TaskBuider<S, M> {
        self.set_closure_option(Some(closure))
    }

    pub fn set_closure_option<S>(self, closure: Option<S>) -> TaskBuider<S, M> {
        TaskBuider {
            closure,
            mapping: self.mapping,
        }
    }

    pub fn set_mapping<S, A, T>(self, mapping: S) -> TaskBuider<C, S>
    where
        S: Fn(A) -> T,
    {
        TaskBuider {
            closure: self.closure,
            mapping: Some(mapping),
        }
    }
}

impl<C, M, V, E> TaskBuider<C, M>
where
    C: Future<Output = Result<V, E>>,
{
    pub fn err_to_arc(self) -> TaskBuider<impl Future<Output = Result<V, Arc<E>>>, M> {
        let closure = self
            .closure
            .map(|closure| async move { closure.await.map_err(Arc::new) });
        TaskBuider {
            closure,
            mapping: self.mapping,
        }
    }
}

impl<C, M, S, T> TaskBuider<C, M>
where
    S: MaybeSend + 'static,
    T: MaybeSend + 'static,
    C: Future<Output = S> + MaybeSend + 'static,
    M: Fn(S) -> T + MaybeSend + 'static,
{
    pub fn perform(self) -> Task<T> {
        let (closure, mapping) = match (self.closure, self.mapping) {
            (Some(closure), Some(mapping)) => (closure, mapping),
            _ => return Task::none(),
        };
        Task::perform(closure, mapping)
    }
}
