use strum::IntoEnumIterator;
use yew::prelude::*;

use base::elements;
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

    pub click: Callback<T>,
}

#[function_component(Buttons)]
pub fn buttons<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let Props {
        class,
        alignment,
        size,
        color,
        value,
        click,
    } = &props;
    let button_map = |variant: T| {
        let selected = value == &variant;
        let color = selected.then(|| color);
        let click = click.reform(move |_| variant);

        html! {
            <elements::Button color={color.cloned()} {click} {selected} size={*size}>
                { variant.to_string() }
            </elements::Button>
        }
    };

    html! {
        <elements::Buttons addons=true alignment={*alignment} class={class.clone()}>
            { for T::iter().map(button_map) }
        </elements::Buttons>
    }
}
