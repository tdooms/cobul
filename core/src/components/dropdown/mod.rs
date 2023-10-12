use yew::prelude::*;

use cobul_props::general::{Active, Hoverable, Right, Up};
pub use divider::DropdownDivider;
pub use item::DropdownItem;

mod divider;
mod item;

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
    pub focus: Callback<bool>,

    #[prop_or_default]
    pub active: Active,

    #[prop_or_default]
    pub fullwidth: bool,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// An interactive dropdown menu for discoverable content - [reference](https://bulma.io/documentation/components/dropdown/)
#[function_component(Dropdown)]
pub fn dropdown(props: &Props) -> Html {
    let class = classes!(
        "dropdown",
        props.class.clone(),
        props.hoverable,
        props.active,
        props.up,
        props.right,
        props.fullwidth.then(|| "is-flex"),
    );

    let onblur = props.focus.reform(|_| false);
    let onmousedown = Callback::from(|e: MouseEvent| e.prevent_default());

    let style = props.fullwidth.then(|| "z-index:100;width:100%");

    html! {
        <div style={props.style.clone()} {class} {onblur}>
            <div class="dropdown-trigger is-clickable" style={style.clone()}>
                { props.trigger.clone() }
            </div>
            <div class="dropdown-menu" role="menu" {style}>
                <div class="dropdown-content" {onmousedown}>
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}