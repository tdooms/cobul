use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_props::{Color, Model, Size};
use cobul_props::general::{Focused, Hovered, Loading, Rounded};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    pub model: Model<T>,

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

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

/// The browser built-in select dropdown, styled accordingly - [reference](https://bulma.io/documentation/form/select/)
///
/// Properties:
/// - `input: Callback<T>` Callback for when the input is changed
/// - `value: Option<T>`
/// - `model: Option<Model<T>>`
/// - `color: Option<Color>`
/// - `size: Option<Size>`
/// - `rounded: Rounded`
/// - `hovered: Hovered`
/// - `focussed: Focused`
/// - `loading: Loading`
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

    let Model { value, input } = props.model.clone();

    let option = move |variant: T| {
        let selected = std::mem::discriminant(&variant) == std::mem::discriminant(&value);
        let onclick = (!selected).then(|| input.reform(move |_| variant));
        html! { <option {selected} {onclick}> {variant.to_string()} </option> }
    };

    html! {
        <div style={props.style.clone()} {class}>
            <select class={classes!(props.hovered, props.focussed)}>
                { for T::iter().map(option) }
            </select>
        </div>
    }
}
