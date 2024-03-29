use yew::prelude::*;

use cobul_props::{Align, Size};
use cobul_props::general::{Boxed, Fullwidth, Toggle, ToggleRounded};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
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

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// Simple responsive horizontal navigation tabs, with different styles - [reference](https://bulma.io/documentation/components/tabs/)
///
/// Properties:
/// - `align: Option<Align>`
/// - `size: Option<Size>`
/// - `boxed: Boxed`
/// - `toggle: Toggle`
/// - `rounded: ToggleRounded`
/// - `fullwidth: Fullwidth`
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
        props.align
    );

    html! {
        <div style={props.style.clone()} {class}>
            <ul> { for props.children.iter() } </ul>
        </div>
    }
}
