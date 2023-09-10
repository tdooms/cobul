use yew::prelude::*;

pub use divider::NavbarDivider;
pub use dropdown::NavbarDropdown;
pub use item::NavbarItem;
pub use link::NavbarLink;

use cobul_props::general::Active;

mod divider;
mod dropdown;
mod item;
mod link;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub brand: Option<Html>,

    #[prop_or_default]
    pub start: Option<Html>,

    #[prop_or_default]
    pub end: Option<Html>,

    #[prop_or_default]
    pub active: Active,

    #[prop_or(true)]
    pub burger: bool,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

/// A responsive horizontal navbar that can support images, links, buttons, and dropdowns - [reference](https://bulma.io/documentation/components/navbar/)
///
/// Properties:
/// - `click: Callback<()>` &npbs; A callback  when the navbar is clicked
/// - `brand: Option<Html>`
/// - `start: Option<Html>`
/// - `end: Option<Html>`
/// - `active: Active`
/// - `burger: bool` &npbs; default: true
#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    let class = classes!("navbar-burger", props.active);
    let onclick = props.click.reform(|_| ());

    let burger = match props.burger {
        true => html! {
            <a role="button" {class} aria-label="menu" aria-expanded="false" data-target="navbar" {onclick}>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
            </a>
        },
        false => html! {},
    };

    let class = classes!("navbar", props.class.clone());

    html! {
        <nav style={props.style.clone()} {class} role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                { props.brand.clone().unwrap_or_default() }
                { burger }
            </div>

            <div id="navbar" class="navbar-menu">
                <div class="navbar-start">
                    { props.start.clone().unwrap_or_default() }
                </div>

                <div class="navbar-end">
                    { props.end.clone().unwrap_or_default() }
                </div>
            </div>
        </nav>
    }
}
