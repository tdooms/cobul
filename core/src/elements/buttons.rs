use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_raw::elements;
use cobul_props::{Align, Color, Size};

#[derive(Clone, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub align: Align,

    #[prop_or_default]
    pub size: Option<Size>,

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
        align,
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
        <elements::Buttons addons=true align={*align} class={class.clone()}>
            { for T::iter().map(button_map) }
        </elements::Buttons>
    }
}