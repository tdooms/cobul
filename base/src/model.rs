use yew::{hook, use_state, Callback};

#[derive(Clone, Debug, PartialEq)]
pub struct Model<T: Clone> {
    pub input: Callback<T>,
    pub value: T,
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
