use std::ops::Deref;

use yew::{Callback, hook, use_state, use_state_eq};

#[derive(Clone, Debug)]
pub struct Model<T: Clone + PartialEq> {
    pub input: Callback<T>,
    pub value: T,
}

impl<T: PartialEq + Clone> PartialEq for Model<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Clone + PartialEq> Model<T> {
    pub fn split(option: &Option<Self>) -> (Option<T>, Callback<T>) {
        match option.clone() {
            Some(Self { value, input }) => (Some(value), input),
            None => (None, Callback::noop())
        }
    }
}

impl Model<bool> {
    pub fn toggle(&self) -> Callback<()> {
        let Model { input, value } = self.clone();
        Callback::from(move |_| input.emit(!value))
    }
}

impl Model<u32> {
    pub fn increment(&self) -> Callback<()> {
        let Model { input, value } = self.clone();
        Callback::from(move |_| input.emit(value + 1))
    }
}

impl<T: Clone + PartialEq> Deref for Model<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[hook]
pub fn use_model<T: Clone + PartialEq + 'static, F: FnOnce() -> T>(f: F) -> Model<T> {
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
