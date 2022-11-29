use yew::prelude::*;

use crate::props::Active;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub close: Callback<()>,

    #[prop_or_default]
    pub title: Option<AttrValue>,

    #[prop_or_default]
    pub footer: Option<Html>,

    #[prop_or_default]
    pub active: Active,

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
/// - `close: Callback<()>`: Callback when the modal is closed
/// - `title: Option<AttrValue>`: The title of the modal
/// - `footer: Option<Html>`: The footer of the modal
/// - `active: Active`: Whether the modal is active or not
#[function_component(ModalCard)]
pub fn modal_card(props: &Props) -> Html {
    let class = classes!("modal", props.class.clone(), props.active);

    let footer = match &props.footer {
        Some(html) => html.clone(),
        None => html! {},
    };

    let close = match props.close == Callback::noop() {
        false => {
            html! { <button class="delete" aria-label="close" onclick={props.close.reform(|_| ())}></button> }
        }
        true => html! {},
    };

    html! {
        <div style={props.style.clone()} {class}>
            <div class="modal-background"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{ props.title.clone() }</p>
                    {close}
                </header>
                <section class="modal-card-body">
                    { for props.children.iter() }
                </section>
                <footer class="modal-card-foot">
                    { footer }
                </footer>
            </div>
        </div>
    }
}
