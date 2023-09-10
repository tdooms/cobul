use yew::prelude::*;

use cobul_props::{HasDropdown, Hoverable};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub dropdown: HasDropdown,

    #[prop_or_default]
    pub hoverable: Hoverable,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

/// A navbar item.
///
/// Properties:
/// - `dropdown: HasDropdown` &npbs; Whether the item has a dropdown.
/// - `hoverable: Hoverable` &npbs; Whether the item is hoverable.
#[function_component(NavbarItem)]
pub fn navbar_item(props: &Props) -> Html {
    let tag = if props.dropdown.0 { "a" } else { "div" };
    let class = classes!(
        "navbar-item",
        props.class.clone(),
        props.dropdown,
        props.hoverable
    );

    html! {
        <@{tag} {class}>
            { for props.children.iter() }
        </@>
    }
}
