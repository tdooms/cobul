use yew::prelude::*;

use crate::props::{Alignment, Boxed, Fullwidth, Rounded, Size, Toggle};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub alignment: Option<Alignment>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub boxed: Boxed,

    #[prop_or_default]
    pub toggle: Toggle,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub fullwidth: Fullwidth,
}


/// [https://bulma.io/documentation/components/tabs/](https://bulma.io/documentation/components/tabs/)
#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    let classes = classes!(
        "tabs",
        &props.extra,
        props.size,
        props.boxed,
        props.toggle,
        props.rounded,
        props.fullwidth
    );

    html! {
        <div class={classes}>
            <ul>
                { for props.children.iter() }
            </ul>
        </div>
    }
}
