use yew::prelude::*;

use cobul_core as core;
use cobul_props::{Color, general::Rounded, Model, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub model: Model<String>,

    #[prop_or_default]
    pub name: Option<AttrValue>,

    #[prop_or_default]
    pub kind: Option<AttrValue>,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let size = props.size.or(use_context::<Size>());
    let color = props.color.or(use_context::<Color>());

    html! {
        <core::Input
            {size}
            {color}
            model={props.model.clone()}
            class={props.class.clone()}
            rounded={props.rounded.clone()}
            name={props.name.clone()}
            kind={props.kind.clone()}
            style={props.style.clone()}
            placeholder={props.placeholder.clone()}
            />
    }
}
