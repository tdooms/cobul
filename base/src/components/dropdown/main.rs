use crate::props::{Hoverable, Active};
use yew::prelude::*;

// TODO: custom trigger without font awesome
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub hoverable: Hoverable,

    #[prop_or_default]
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub active: Active,

    pub text: String,
}

/// [// https://bulma.io/documentation/components/dropdown/](// https://bulma.io/documentation/components/dropdown/)
#[function_component(Dropdown)]
pub fn dropdown(props: &Props) -> Html {
    let classes = classes!(
        "dropdown",
        &props.extra,
        props.hoverable,
        props.active
    );
    let onclick = props.onclick.reform(|_| ());

    html! {
        <div class={classes}>
            <div class="dropdown-trigger">
                <button class="button" aria-haspopup="true" onclick={onclick}>
                    <span> {props.text.clone()} </span>
                    <span class="icon is-small"> <i class="fas fa-angle-down" aria-hidden="true"></i> </span>
                </button>
            </div>
            <div class="dropdown-menu" role="menu">
                <div class="dropdown-content">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
