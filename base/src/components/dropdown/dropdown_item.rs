use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: Routable + PartialEq + 'static> {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    route: T,
}

/// [// https://bulma.io/documentation/components/dropdown/](// https://bulma.io/documentation/components/dropdown/)
#[function_component(DropdownItem)]
pub fn dropdown_item<T: Routable + PartialEq + 'static>(props: &Props<T>) -> Html {
    let classes = classes!("dropdown-item", props.class.clone());
    html! {
        <Link<T> classes={classes} to={props.route.clone()}>
            { for props.children.iter() }
        </Link<T>>
    }
}
