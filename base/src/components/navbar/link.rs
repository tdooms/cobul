use yew::prelude::*;
use yew_router::{prelude::Link, Routable};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: Routable + PartialEq + 'static> {
    route: T,

    #[prop_or_default]
    children: Children,
}

/// [https://bulma.io/documentation/components/navbar/](https://bulma.io/documentation/components/navbar/)
#[function_component(NavbarLink)]
pub fn navbar_link<T>(props: &Props<T>) -> Html
where
    T: Routable + PartialEq + 'static,
{
    html! {
        <Link<T> classes={classes!("navbar-link")} to={props.route.clone()}>
            { for props.children.iter() }
        </Link<T>>
    }
}
