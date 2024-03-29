use yew::prelude::*;

use cobul_props::ContainerSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub size: Option<ContainerSize>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// A simple container to center your content horizontally - [reference](https://bulma.io/documentation/layout/container/)
///
/// Properties:
/// - `size: Option<ContainerSize>`
#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let class = classes!("container", props.class.clone(), props.size);

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
