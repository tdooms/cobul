use yew::*;

use crate::props::{Alignment, Current, Disabled, Rounded, Size};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,
}

/// A responsive, usable, and flexible pagination - [reference](https://bulma.io/documentation/components/pagination/)
#[function_component(Pagination)]
pub fn pagination(props: &Props) -> Html {
    let class = classes!(
        props.rounded,
        props.alignment,
        props.size,
        "pagination",
        props.class.clone()
    );

    html! {
        <nav {class} role="navigation" aria-label="pagination" style={props.style.clone()}>
            { for props.children.iter() }
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct SubProps {
    pub children: Children,
}

#[derive(Properties, PartialEq)]
pub struct NavProps {
    #[prop_or_default]
    pub disabled: Disabled,

    #[prop_or_default]
    pub children: Children,

    pub click: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub click: Callback<()>,

    #[prop_or_default]
    pub current: Current,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(PaginationNext)]
pub fn pagination_next(props: &NavProps) -> Html {
    let class = classes!("pagination-next", props.disabled);
    html! {<a {class} onclick={props.click.reform(|_| ())}> { for props.children.iter() } </a>}
}

#[function_component(PaginationPrevious)]
pub fn pagination_previous(props: &NavProps) -> Html {
    let class = classes!("pagination-previous", props.disabled);
    html! { <a {class} onclick={props.click.reform(|_| ())}> { for props.children.iter() } </a> }
}

#[function_component(PaginationList)]
pub fn pagination_list(props: &SubProps) -> Html {
    html! { <ul class="pagination-list"> { for props.children.iter() } </ul> }
}

#[function_component(PaginationLink)]
pub fn pagination_link(props: &LinkProps) -> Html {
    let class = classes!("pagination-link", props.current);
    html! { <li><a {class} onclick={props.click.reform(|_| ())}> { for props.children.iter() } </a></li> }
}

#[function_component(PaginationEllipsis)]
pub fn pagination_ellipsis(props: &SubProps) -> Html {
    html! { <li><span class="pagination-ellipsis"> { for props.children.iter() } </span></li> }
}
