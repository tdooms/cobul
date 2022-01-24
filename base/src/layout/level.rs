use yew::prelude::*;

use crate::props::{Mobile, TextCentered};
use crate::utils::enclose;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub left: Option<Html>,

    #[prop_or_default]
    pub right: Option<Html>,

    #[prop_or_default]
    pub centered: TextCentered,

    #[prop_or_default]
    pub mobile: Mobile,
}

/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
#[function_component(Level)]
pub fn level(props: &Props) -> Html {
    let classes = classes!("level", props.class.clone(), props.centered, props.mobile);
    html! {
        <div class={classes}>
            { enclose("media-left", props.left.clone()) }
            { for props.children.iter() }
            { enclose("media-right", props.right.clone()) }
        </div>
    }
}
