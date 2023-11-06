use yew::prelude::*;

use cobul_props::general::Active;
use cobul_model::Model;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub model: Model<bool>,

    #[prop_or_default]
    pub close: bool,

    #[prop_or_default]
    pub title: Option<AttrValue>,

    #[prop_or_default]
    pub footer: Option<Html>,

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
#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    let class = classes!("modal", props.class.clone(), Active(props.model.value()));

    let onclick = props.model.reform(|_| false);
    let close = match (props.footer.is_some() || props.title.is_some(), props.close) {
        (true, true) => html! { <button class="delete" {onclick}></button> },
        (false, true) => html! { <button class="modal-close" {onclick}></button> },
        (_, false) => html! {},
    };

    let inner = match (props.footer.clone(), props.title.clone()) {
        (None, None) => html! {
            <>
            <div class="modal-content" style={props.style.clone()}> { for props.children.iter() } </div>
            {close}
            </>
        },
        _ => html! {
            <div class="modal-card" style={props.style.clone()}>
            <header class="modal-card-head">
                <p class="modal-card-title">{ props.title.clone() }</p>
                {close}
            </header>
            <section class="modal-card-body">
                { for props.children.iter() }
            </section>
            <footer class="modal-card-foot">
                { props.footer.clone() }
            </footer>
            </div>
        }
    };

    html! {
        <div {class}>
            <div class="modal-background" />
            {inner}
        </div>
    }
}
