use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    children: Children,
}

/// [https://bulma.io/documentation/components/navbar/](https://bulma.io/documentation/components/navbar/)
#[function_component(NavbarDropdown)]
pub fn navbar_dropdown(props: &Props) -> Html {
    html! {
        <div class="navbar-dropdown">
            { for props.children.iter() }
        </div>
    }
}
