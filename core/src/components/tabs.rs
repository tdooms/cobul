use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_raw::{components};
use cobul_props::{Align, Size, Model};
use cobul_props::general::{Boxed, Fullwidth, Toggle, ToggleRounded};

#[derive(Clone, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub value: Option<T>,

    #[prop_or_default]
    pub input: Callback<T>,

    #[prop_or_default]
    pub model: Option<Model<T>>,

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

    let (input, value) = Model::combine(&props.input, &props.value, &props.model);

    let tab_map = |variant: T| {
        let class = (&value == &Some(variant)).then(|| "is-active");
        let onclick = input.reform(move |_| variant);

        html! { <li {onclick} {class}> <a> <span> { variant.to_string() } </span> </a> </li> }
    };

    html! {
        <components::Tabs {class} {align} {size} {boxed} {toggle} {rounded} {fullwidth} {style}>
           <ul> { for T::iter().map(tab_map) } </ul>
        </components::Tabs>
    }
}
