use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_props::{Align, Color, Size};
use cobul_model::Model;
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
    let Props { class, align, size, color, model } = props.clone();

    let button_map = |variant: T| {
        let selected = &*model == &variant;
        let color = selected.then(|| color).flatten();
        let click = model.reform(move |_| variant);

        html! { <core::Button {color} {click} {selected} {size} text={variant.to_string()} /> }
    };

    html! {
        <core::Buttons addons=true {align} class={class.clone()}>
            { for T::iter().map(button_map) }
        </core::Buttons>
    }
}
