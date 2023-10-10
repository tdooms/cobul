use yew::prelude::*;

pub use block::PanelBlock;
use cobul_props::Color;
pub use tabs::PanelTabs;

use crate::util::enclose_tag;

mod block;
mod tabs;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub heading: Option<Html>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A composable panel, for compact controls - [reference](https://bulma.io/documentation/components/panel/)
///
/// Properties:
/// - `color: Option<Color>`
/// - `heading: Option<Html>`
#[function_component(Panel)]
pub fn panel(props: &Props) -> Html {
    let class = classes!("panel", props.class.clone(), props.color);
    html! {
        <nav style={props.style.clone()} {class}>
            { enclose_tag("p", "panel-heading", props.heading.clone()) }
            { for props.children.iter() }
        </nav>
    }
}
