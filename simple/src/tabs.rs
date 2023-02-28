use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_base::{components, elements};
use cobul_base::model::Model;
use cobul_base::props::{Alignment, Boxed, Fullwidth, Size, Toggle, ToggleRounded};
use cobul_base::utils::combine_model;

pub trait HasIcon {
    fn icon(&self) -> Option<String>;
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + HasIcon + 'static> {
    #[prop_or_default]
    pub value: Option<T>,

    #[prop_or_default]
    pub input: Callback<T>,

    #[prop_or_default]
    pub model: Option<Model<T>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub alignment: Option<Alignment>,

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
    T: IntoEnumIterator + ToString + Copy + PartialEq + HasIcon + 'static,
{
    let Props {
        class,
        alignment,
        size,
        boxed,
        toggle,
        rounded,
        fullwidth,
        style,
        ..
    } = props.clone();

    let (input, value) = combine_model(&props.input, &props.value, &props.model);

    let tab_map = |variant: T| {
        let class = (&value == &Some(variant)).then(|| "is-active");
        let onclick = input.reform(move |_| variant);
        let icon = variant.icon().map(|icon| html! {<elements::Icon {icon} class="m-0"/>});

        html! { <li {onclick} {class}> <a> {icon} <span> { variant.to_string() } </span> </a> </li> }
    };

    html! {
        <components::Tabs {class} {alignment} {size} {boxed} {toggle} {rounded} {fullwidth} {style}>
           <ul> { for T::iter().map(tab_map) } </ul>
        </components::Tabs>
    }
}
