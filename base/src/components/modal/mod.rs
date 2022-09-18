use yew::prelude::*;

pub use card::ModalCard;

use crate::props::Active;

mod card;

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
    pub style: Option<AttrValue>,
}

/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    let class = classes!("modal", props.class.clone(), props.active);
    let width = props.width.map(|x| format!("width:{}px !important;", x));

    html! {
        <div style={props.style.clone()} {class}>
            <div class="modal-background"></div>
            <div class="modal-content" style={width}>
                { for props.children.iter() }
            </div>
        </div>
    }
}
