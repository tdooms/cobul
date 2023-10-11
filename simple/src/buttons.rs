use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_props::{Align, Color, Model, Size};
use cobul_core as core;

#[derive(Clone, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    pub model: Model<T>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub align: Option<Align>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<Color>,
}

#[function_component(Buttons)]
pub fn buttons<T>(props: &Props<T>) -> Html
    where
        T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let Props { class, align, size, color, model } = &props;
    let Model { value, input } = model.clone();

    let button_map = |variant: T| {
        let selected = &value == &variant;
        let color = selected.then(|| color).cloned().flatten();
        let click = input.reform(move |_| variant);

        html! { <core::Button {color} {click} {selected} size={*size} text={variant.to_string()} /> }
    };

    html! {
        <core::Buttons addons=true align={*align} class={class.clone()}>
            { for T::iter().map(button_map) }
        </core::Buttons>
    }
}
