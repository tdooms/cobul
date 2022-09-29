use crate::model::Model;
use yew::prelude::*;

pub fn enclose(class: impl Into<String>, option: Option<Html>) -> Html {
    html! { <div class={class.into()}> {option.unwrap_or_default()} </div> }
}

pub fn enclose_tag(tag: impl Into<String>, class: impl Into<String>, opt: Option<Html>) -> Html {
    html! { <@{tag.into()} class={class.into()}> {opt.unwrap_or_default()} </@> }
}

pub fn combine_model<T: Clone>(
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
