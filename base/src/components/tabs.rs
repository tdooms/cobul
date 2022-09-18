use yew::prelude::*;

use crate::props::{Alignment, Boxed, Fullwidth, Size, Toggle, ToggleRounded};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

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

/// [https://bulma.io/documentation/components/tabs/](https://bulma.io/documentation/components/tabs/)
#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    let class = classes!(
        "tabs",
        props.class.clone(),
        props.size,
        props.boxed,
        props.toggle,
        props.rounded,
        props.fullwidth,
        props.alignment
    );

    html! {
        <div style={props.style.clone()} {class}>
            <ul>
                { for props.children.iter() }
            </ul>
        </div>
    }
}
