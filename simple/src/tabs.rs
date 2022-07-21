use strum::IntoEnumIterator;
use yew::prelude::*;

use base::components;
use base::props::{Alignment, Size};

#[derive(Clone, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub alignment: Option<Alignment>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub boxed: bool,

    #[prop_or_default]
    pub toggle: bool,

    #[prop_or_default]
    pub rounded: bool,

    #[prop_or_default]
    pub fullwidth: bool,

    #[prop_or_default]
    pub style: Option<String>,

    pub value: T,

    pub onclick: Callback<T>,
}

#[function_component(Tabs)]
pub fn tabs<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let tab_map = |variant: T| {
        let class = (&props.value == &variant).then(|| "is-active");
        let onclick = props.onclick.reform(move |_| variant);

        html! {
            <li {onclick} {class}>
                <a> { variant.to_string() } </a>
            </li>
        }
    };

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

    html! {
        <components::Tabs {class} {alignment} {size} {boxed} {toggle} {rounded} {fullwidth} {style}>
           <ul> { for T::iter().map(tab_map) } </ul>
        </components::Tabs>
    }
}
