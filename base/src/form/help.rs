use yew::prelude::*;

use crate::props::Color;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// All generic form controls, designed for consistency - [reference](https://bulma.io/documentation/form/general/)
#[function_component(Help)]
pub fn help(props: &Props) -> Html {
    let class = classes!("help", props.class.clone(), props.color);

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
