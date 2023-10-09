use crate::FormData;
use cobul_props::general::Rounded;
use cobul_props::{Model, Size};
use cobul_raw::form;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input: Callback<String>,

    #[prop_or_default]
    pub value: Option<String>,

    #[prop_or_default]
    pub model: Option<Model<String>>,

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
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let _form = use_context::<FormData>();
    let size = props.size.or(use_context::<Size>());

    html! {
        <form::Input
            value={props.value.clone()}
            input={props.input.clone()}
            model={props.model.clone()}
            class={props.class.clone()}
            size={size}
            rounded={props.rounded.clone()}
            name={props.name.clone()}
            kind={props.kind.clone()}
            style={props.style.clone()}
            placeholder={props.placeholder.clone()}
            />
    }
}
