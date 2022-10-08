use yew::prelude::*;

use crate::props::{HasDropdown, Hoverable};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    dropdown: HasDropdown,

    #[prop_or_default]
    hoverable: Hoverable,
}

#[function_component(NavbarItem)]
pub fn navbar_item(props: &Props) -> Html {
    let tag = if props.dropdown.0 { "a" } else { "div" };
    let class = classes!("navbar-item", props.dropdown, props.hoverable);

    html! {
        <@{tag} {class}>
            { for props.children.iter() }
        </@>
    }
}
