use yew::prelude::*;

use crate::props::Color;
use crate::utils::enclose_with_tag;

mod block;
mod tabs;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub heading: Option<Html>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[function_component(Panel)]
pub fn panel(props: &Props) -> Html {
    let class = classes!("panel", props.class.clone(), props.color);
    html! {
        <nav style={props.style.clone()} {class}>
            { enclose_with_tag("p", "panel-heading", props.heading.clone()) }
            { for props.children.iter() }
        </nav>
    }
}
