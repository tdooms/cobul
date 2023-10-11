use std::ops::Deref;

use yew::{Callback, hook, use_state, use_state_eq};

#[derive(Clone, Debug, PartialEq)]
pub struct Model<T: Clone> {
    pub input: Callback<T>,
    pub value: T,
}

impl<T: Clone> Model<T> {
    pub fn constant(value: T) -> Model<T> {
        Model { input: Callback::noop(), value }
    }
}

impl<T: Clone> Deref for Model<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[hook]
pub fn use_model<T: Clone + 'static, F: FnOnce() -> T>(f: F) -> Model<T> {
    let state = use_state(f);
    let cloned = state.clone();

    Model {
        input: Callback::from(move |new| cloned.set(new)),
        value: (*state).clone(),
    }
}

#[hook]
pub fn use_model_eq<T: Clone + PartialEq + 'static, F: FnOnce() -> T>(f: F) -> Model<T> {
    let state = use_state_eq(f);
    let cloned = state.clone();

    Model {
        input: Callback::from(move |new| cloned.set(new)),
        value: (*state).clone(),
    }
}

