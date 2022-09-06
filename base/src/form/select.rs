use strum::IntoEnumIterator;
use yew::prelude::*;

use crate::props::{Color, Focused, Hovered, Loading, Rounded, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub hovered: Hovered,

    #[prop_or_default]
    pub focussed: Focused,

    #[prop_or_default]
    pub loading: Loading,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub input: Callback<T>,

    pub value: T,
}

/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
#[function_component(Select)]
pub fn select<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let classes = classes!(
        "select",
        props.class.clone(),
        props.color,
        props.size,
        props.rounded,
        props.loading
    );

    let view_option = |variant: T| {
        let value = std::mem::discriminant(&variant) == std::mem::discriminant(&props.value);
        let click = (!value).then(move || props.input.reform(move |_| variant));
        html! { <option selected={value} onclick={click}> {variant} </option> }
    };

    html! {
        <div style={props.style.clone()} class={classes}>
            <select class={classes!(props.hovered, props.focussed)}>
                { for T::iter().map(view_option) }
            </select>
        </div>
    }
}
