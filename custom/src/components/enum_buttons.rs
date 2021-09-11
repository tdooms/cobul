use strum::IntoEnumIterator;
use yew::prelude::*;

use base::elements::{Button, Buttons};
use base::props::{Alignment, Color};

#[derive(Clone, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub alignment: Alignment,

    pub color: Color,

    pub value: T,

    pub onclick: Callback<T>,
}

#[function_component(EnumButtons)]
pub fn enum_buttons<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static>(
    props: &Props<T>,
) -> Html {
    let button_map = |variant: T| {
        let selected = props.value == variant;
        let color = selected.then(|| props.color);
        let onclick = props.onclick.reform(move |_| variant);

        html! {
            <Button color={color} onclick={onclick} selected={selected}>
                {variant.to_string()}
            </Button>
        }
    };

    html! {
        <Buttons addons=true alignment={props.alignment}>
            { for T::iter().map(button_map) }
        </Buttons>
    }
}
