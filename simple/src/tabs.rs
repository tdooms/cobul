use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_props::{Align, Size};
use cobul_model::Model;
use cobul_props::general::{Boxed, Fullwidth, Toggle, ToggleRounded};
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
    pub boxed: Boxed,

    #[prop_or_default]
    pub toggle: Toggle,

    #[prop_or_default]
    pub rounded: ToggleRounded,

    #[prop_or_default]
    pub fullwidth: Fullwidth,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Tabs)]
pub fn tabs<T>(props: &Props<T>) -> Html
    where
        T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let Props {
        class,
        align,
        size,
        boxed,
        toggle,
        rounded,
        fullwidth,
        style,
        ..
    } = props.clone();
    let model = props.model.clone();

    let tab_map = |variant: T| {
        let class = (&*model == &variant).then(|| "is-active");
        let onclick = model.reform(move |_| variant);

        html! { <li {onclick} {class}> <a> <span> { variant.to_string() } </span> </a> </li> }
    };

    html! {
        <core::Tabs {class} {align} {size} {boxed} {toggle} {rounded} {fullwidth} {style}>
           <ul> { for T::iter().map(tab_map) } </ul>
        </core::Tabs>
    }
}
