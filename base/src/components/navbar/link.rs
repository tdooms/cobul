use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    click: Callback<()>,

    #[prop_or_default]
    children: Children,
}

#[function_component(NavbarLink)]
pub fn navbar_link<T>(props: &Props) -> Html {
    html! {
        <a classes={classes!("navbar-link")} onclick={props.click.reform(|_| ())}>
            { for props.children.iter() }
        </a>
    }
}
