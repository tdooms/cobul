use yew::{hook, use_state, Callback};

#[derive(Clone, Debug, PartialEq)]
pub struct Model<T: Clone> {
    pub input: Callback<T>,
    pub value: T,
}

impl<T: Clone> Model<T> {
    pub fn combine(input: &Callback<T>, value: &Option<T>, model: &Option<Model<T>>) -> (Callback<T>, Option<T>) {
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

#[hook]
pub fn use_model<T: Clone + 'static, F: FnOnce() -> T>(f: F) -> Model<T> {
    let state = use_state(f);
    let cloned = state.clone();

    Model {
        input: Callback::from(move |new| cloned.set(new)),
        value: (*state).clone(),
    }
}
