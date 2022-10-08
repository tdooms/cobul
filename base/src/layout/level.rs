use yew::prelude::*;

use crate::props::{Mobile, TextCentered};
use crate::utils::enclose;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelProps {
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

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A multi-purpose horizontal level, which can contain almost any other element - [reference](https://bulma.io/documentation/layout/level/)
#[function_component(Level)]
pub fn level(props: &LevelProps) -> Html {
    let class = classes!("level", props.class.clone(), props.centered, props.mobile);
    html! {
        <div style={props.style.clone()} {class}>
            { enclose("media-left", props.left.clone()) }
            { for props.children.iter() }
            { enclose("media-right", props.right.clone()) }
        </div>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelItemProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// [reference](https://bulma.io/documentation/layout/level/)
#[function_component(LevelItem)]
pub fn level_item(props: &LevelItemProps) -> Html {
    let class = classes!("level-item", props.class.clone());
    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
