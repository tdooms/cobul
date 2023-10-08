use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_props::{Align, Color, Size, Model};
use cobul_raw::elements;

#[derive(Clone, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub align: Option<Align>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub value: Option<T>,

    #[prop_or_default]
    pub input: Callback<T>,

    #[prop_or_default]
    pub model: Option<Model<T>>,
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
        input,
        model
    } = &props;

    let (input, value) = Model::combine(input, value, model);

    let button_map = |variant: T| {
        let selected = &value.unwrap() == &variant;
        let color = selected.then(|| color).cloned().flatten();
        let click = input.reform(move |_| variant);

        html! {
            <elements::Button {color} {click} {selected} size={*size}>
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
