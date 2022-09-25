use yew::Callback;

#[derive(Clone, Debug, PartialEq)]
pub struct Model<T> {
    pub input: Callback<T>,
    pub value: T,
}
