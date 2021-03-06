use yew::prelude::*;

use crate::props::Active;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    pub title: String,

    #[prop_or_default]
    pub footer: Option<Html>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub active: Active,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
}

/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
#[function_component(ModalCard)]
pub fn modal_card(props: &Props) -> Html {
    let classes = classes!("modal", props.class.clone(), props.active);

    let footer = match &props.footer {
        Some(html) => html.clone(),
        None => html! {},
    };

    let close = match props.onclose.as_ref().map(|x| x.reform(|_| ())) {
        Some(onclick) => html! {<button class="delete" aria-label="close" {onclick}></button>},
        None => html! {}
    };

    html! {
        <div style={props.style.clone()} class={classes}>
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
