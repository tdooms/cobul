use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    children: Children,
}

#[function_component(NavbarDivider)]
pub fn navbar_divider(props: &Props) -> Html {
    html! {
        <div class="navbar-divider">
            { for props.children.iter() }
        </div>
    }
}
