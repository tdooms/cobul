use yew::prelude::*;

use crate::props::Addons;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub addons: Addons,
}

/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
#[function_component(Tags)]
pub fn tags(props: &Props) -> Html {
    let classes = classes!("tags", props.class.clone(), props.addons);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
