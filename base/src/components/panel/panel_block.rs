use crate::props::Active;
use yew::prelude::*;

// TODO: this can only be: control, input, button, panel-icon
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub active: Active,
}

/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[function_component(PanelBlock)]
pub fn panel_block(props: &Props) -> Html {
    let classes = classes!("panel-block", props.class.clone(), props.active);
    html! {
        <nav class={classes}>
            { for props.children.iter() }
        </nav>
    }
}
