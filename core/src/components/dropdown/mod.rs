use yew::prelude::*;
use cobul_model::Model;

use cobul_props::general::{Active, Hoverable, Right, Up};
pub use divider::DropdownDivider;
pub use item::DropdownItem;

mod divider;
mod item;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub model: Model<bool>,
    pub trigger: Html,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub up: Up,

    #[prop_or_default]
    pub right: Right,

    #[prop_or_default]
    pub hoverable: Hoverable,

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
        Active(props.model.value),
        props.up,
        props.right,
        props.fullwidth.then(|| "is-flex"),
    );

    let onblur = props.model.reform(|_| false);
    let onmousedown = Callback::from(|e: MouseEvent| e.prevent_default());

    let style = props.fullwidth.then(|| "width:100%").unwrap_or_default();

    html! {
        <div style={props.style.clone()} {class} {onblur}>
            <div class="dropdown-trigger is-clickable" style={style.clone()}>
                { props.trigger.clone() }
            </div>
            <div class="dropdown-menu" role="menu" style={format!("z-index:100;{style}")}>
                <div class="dropdown-content" {onmousedown}>
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
