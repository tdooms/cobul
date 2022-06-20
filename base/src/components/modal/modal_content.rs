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
    pub style: String,
}

/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
#[function_component(ModalContent)]
pub fn modal_content(props: &Props) -> Html {
    let classes = classes!("modal", props.class.clone(), props.active);

    html! {
        <div style={props.style.clone()} class={classes}>
            <div class="modal-background"></div>
            <div class="modal-content">
                { for props.children.iter() }
            </div>
        </div>
    }
}
