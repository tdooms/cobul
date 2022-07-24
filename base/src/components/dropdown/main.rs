use crate::props::{Active, Hoverable, Right, Up};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    pub trigger: Html,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub up: Up,

    #[prop_or_default]
    pub right: Right,

    #[prop_or_default]
    pub hoverable: Hoverable,

    #[prop_or_default]
    pub onfocus: Callback<bool>,

    #[prop_or_default]
    pub active: Active,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [// https://bulma.io/documentation/components/dropdown/](// https://bulma.io/documentation/components/dropdown/)
#[function_component(Dropdown)]
pub fn dropdown(props: &Props) -> Html {
    let class = classes!(
        "dropdown",
        props.class.clone(),
        props.hoverable,
        props.active,
        props.up,
        props.right
    );

    let onfocus = props.onfocus.reform(|_| true);
    let onblur = props.onfocus.reform(|_| false);
    let onmousedown = Callback::from(|e: MouseEvent| e.prevent_default());

    let style = props.style.clone();
    html! {
        <div {style} {class} onclick={onfocus.clone()} {onblur}>
            <div class="dropdown-trigger is-clickable">
                { props.trigger.clone() }
            </div>
            <div class="dropdown-menu" role="menu">
                <div class="dropdown-content" {onmousedown}>
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
