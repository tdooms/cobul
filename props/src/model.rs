use std::ops::Deref;
use yew::{hook, use_state, Callback, use_state_eq};

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
    pub fn combine(
        input: &Callback<T>,
        value: &Option<T>,
        model: &Option<Model<T>>,
    ) -> (Callback<T>, Option<T>) {
        let input = match (input.clone(), model.as_ref().map(|x| x.input.clone())) {
            (x, Some(input)) if x != Callback::noop() => input,
            (input, _) => input,
        };
        let value = match (value.clone(), model.as_ref().map(|x| x.value.clone())) {
            (Some(value), _) => Some(value),
            (None, value) => value,
        };
        (input, value)
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
