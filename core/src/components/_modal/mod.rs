pub use card::ModalCard;
use cobul_props::general::Active;
use yew::prelude::*;

mod card;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub active: Active,

    #[prop_or_default]
    pub width: Option<u32>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// A classic modal overlay, in which you can include any content you want - [reference](https://bulma.io/documentation/components/modal/)
///
/// Properties:
/// - `active: Active` &npbs; Whether the modal is active or not
/// - `width: Option<u32>` &npbs; The width of the modal
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
