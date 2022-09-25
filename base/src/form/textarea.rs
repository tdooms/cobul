use crate::model::Model;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::props::{Color, Disabled, FixedSize, Loading, Readonly, Size, Static};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub value: Option<String>,

    #[prop_or_default]
    pub input: Callback<String>,

    #[prop_or_default]
    pub model: Option<Model<String>>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub name: Option<String>,

    #[prop_or_default]
    pub rows: Option<u32>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub fixed_size: FixedSize,

    #[prop_or_default]
    pub loading: Loading,

    #[prop_or_default]
    pub disabled: Disabled,

    #[prop_or_default]
    pub readonly: Readonly,

    #[prop_or_default]
    pub fixed: Static,
}

/// [https://bulma.io/documentation/form/textarea/](https://bulma.io/documentation/form/textarea/)
#[function_component(Textarea)]
pub fn textarea(props: &Props) -> Html {
    let class = classes!(
        "textarea",
        props.class.clone(),
        props.color,
        props.size,
        props.loading,
        props.fixed,
        props.fixed_size
    );

    let reform = |e: InputEvent| e.target_unchecked_into::<HtmlInputElement>().value();

    let (oninput, value) = match &props.model {
        Some(Model { input, value }) => (input.reform(reform), Some(value.clone())),
        None => (props.input.reform(reform), props.value.clone()),
    };

    html! {
        <textarea

            {value}
            {oninput}
            {class}
            name={props.name.clone()}
            style={props.style.clone()}
            rows={props.rows.as_ref().map(ToString::to_string)}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled.0}
            readonly={props.readonly.0}
            />
    }
}
