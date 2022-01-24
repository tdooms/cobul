use yew::prelude::*;

use crate::props::ContainerSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub size: Option<ContainerSize>,
}

/// [https://bulma.io/documentation/layout/container/](https://bulma.io/documentation/layout/container/)
#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let classes = classes!("container", props.class.clone(), props.size);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
