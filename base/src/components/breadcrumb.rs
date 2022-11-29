use yew::prelude::*;

use crate::props::{Alignment, Separator, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub alignment: Option<Alignment>,

    #[prop_or_default]
    pub separator: Option<Separator>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// A simple breadcrumb component to improve your navigation experience - [reference](https://bulma.io/documentation/components/breadcrumb/)
///
/// Properties:
/// - `size: Option<Size>` &npbs; The size of the breadcrumb
/// - `alignment: Option<Alignment>` &npbs; The alignment of the breadcrumb
/// - `separator: Option<Separator>` &npbs; The separator of the breadcrumb
#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &Props) -> Html {
    let class = classes!(
        "breadcrumb",
        props.size,
        props.alignment,
        props.separator,
        props.class.clone()
    );

    html! {
        <nav style={props.style.clone()} {class} aria-label="breadcrumbs">
            <ul>
                { for props.children.iter() }
            </ul>
        </nav>
    }
}
