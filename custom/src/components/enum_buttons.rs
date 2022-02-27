use strum::IntoEnumIterator;
use yew::prelude::*;

use base::elements::{Button, Buttons};
use base::props::{Alignment, Color, Size};

#[derive(Clone, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub size: Size,

    pub color: Color,

    pub value: T,

    pub onclick: Callback<T>,
}

#[function_component(EnumButtons)]
pub fn enum_buttons<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let Props {
        class,
        alignment,
        size,
        color,
        value,
        onclick,
    } = &props;
    let button_map = |variant: T| {
        let selected = value == &variant;
        let color = selected.then(|| color);
        let onclick = onclick.reform(move |_| variant);

        html! {
            <Button color={color.cloned()} onclick={onclick} selected={selected} size={*size}>
                { variant.to_string() }
            </Button>
        }
    };

    html! {
        <Buttons addons=true alignment={*alignment} class={class.clone()}>
            { for T::iter().map(button_map) }
        </Buttons>
    }
}
