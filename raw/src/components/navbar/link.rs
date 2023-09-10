use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

/// A simple component for rendering a navbar link.
///
/// Properties:
/// - `click: Callback<()>` &npbs; A callback that is invoked when the link is clicked.
#[function_component(NavbarLink)]
pub fn navbar_link<T>(props: &Props) -> Html {
    html! {
        <a classes={classes!("navbar-link")} onclick={props.click.reform(|_| ())}>
            { for props.children.iter() }
        </a>
    }
}
