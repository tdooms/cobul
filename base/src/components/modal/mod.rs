pub use card::ModalCard;
mod card;

use yew::prelude::*;

use crate::props::Active;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub active: Active,

    #[prop_or_default]
    pub width: Option<u64>,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    let classes = classes!("modal", props.class.clone(), props.active);
    let width = props.width.map(|x| format!("width:{}px !important;", x));

    html! {
        <div style={props.style.clone()} class={classes}>
            <div class="modal-background"></div>
            <div class="modal-content" style={width}>
                { for props.children.iter() }
            </div>
        </div>
    }
}
