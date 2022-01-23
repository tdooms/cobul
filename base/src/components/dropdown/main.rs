use crate::props::{Hoverable, Active, Up, Right};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    pub trigger: Html,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub up: Up,

    #[prop_or_default]
    pub right: Right,

    #[prop_or_default]
    pub hoverable: Hoverable,

    #[prop_or_default]
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub active: Active,
}

/// [// https://bulma.io/documentation/components/dropdown/](// https://bulma.io/documentation/components/dropdown/)
#[function_component(Dropdown)]
pub fn dropdown(props: &Props) -> Html {
    let classes = classes!(
        "dropdown",
        &props.extra,
        props.hoverable,
        props.active,
        props.up,
        props.right
    );
    let onclick = props.onclick.reform(|_| ());

    html! {
        <div class={classes} onclick={onclick}>
            <div class="dropdown-trigger is-clickable">
                { props.trigger.clone() }
            </div>
            <div class="dropdown-menu" role="menu">
                <div class="dropdown-content">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
