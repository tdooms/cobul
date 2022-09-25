use crate::model::Model;
use strum::IntoEnumIterator;
use yew::prelude::*;

use crate::props::{Color, Focused, Hovered, Loading, Rounded, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub input: Callback<T>,

    pub value: T,

    #[prop_or_default]
    pub model: Option<Model<T>>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub hovered: Hovered,

    #[prop_or_default]
    pub focussed: Focused,

    #[prop_or_default]
    pub loading: Loading,
}

/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
#[function_component(Select)]
pub fn select<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let class = classes!(
        "select",
        props.class.clone(),
        props.color,
        props.size,
        props.rounded,
        props.loading
    );

    let (input, value) = match props.model.clone() {
        Some(Model { input, value }) => (input, value),
        _ => (props.input.clone(), props.value.clone()),
    };

    let option = move |variant: T| {
        let selected = std::mem::discriminant(&variant) == std::mem::discriminant(&value);
        let onclick = (!selected).then(|| input.reform(move |_| variant));
        html! { <option {selected} {onclick}> {variant} </option> }
    };

    html! {
        <div style={props.style.clone()} {class}>
            <select class={classes!(props.hovered, props.focussed)}>
                { for T::iter().map(option) }
            </select>
        </div>
    }
}
