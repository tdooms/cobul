use yew::prelude::*;

use cobul_props::Active;

// TODO: this can only be: control, input, button, panel-icon
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub active: Active,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// A simple component for rendering a panel block.
///
/// Properties:
/// - `active: Active` &npbs; Whether the block is active.
#[function_component(PanelBlock)]
pub fn panel_block(props: &Props) -> Html {
    let class = classes!("panel-block", props.class.clone(), props.active);
    html! {
        <nav style={props.style.clone()} {class}>
            { for props.children.iter() }
        </nav>
    }
}
