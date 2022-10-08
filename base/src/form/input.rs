use crate::model::Model;
use crate::props::{Color, Disabled, Loading, Readonly, Rounded, Size, Static};
use crate::utils::combine_model;
use web_sys::HtmlInputElement;
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
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub name: Option<AttrValue>,

    #[prop_or_default]
    pub kind: Option<AttrValue>,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub loading: Loading,

    #[prop_or_default]
    pub disabled: Disabled,

    #[prop_or_default]
    pub readonly: Readonly,

    #[prop_or_default]
    pub statik: Static,
}

/// The text input and its variations - [reference](https://bulma.io/documentation/form/input/)
#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let size = use_context::<Size>();
    let color = use_context::<Color>();
    let rounded = use_context::<Rounded>();
    let loading = use_context::<Loading>();
    let statik = use_context::<Static>();

    let class = classes!(
        "input",
        props.class.clone(),
        props.size.or(size),
        props.color.or(color),
        props.rounded.or(rounded),
        props.loading.or(loading),
        props.statik.or(statik),
    );

    let (input, value) = combine_model(&props.input, &props.value, &props.model);
    let reform = |e: InputEvent| e.target_unchecked_into::<HtmlInputElement>().value();
    let oninput = input.reform(reform);

    html! {
        <input
            {value}
            {oninput}
            {class}
            name={props.name.clone()}
            type={props.kind.clone()}
            style={props.style.clone()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled.0}
            readonly={props.readonly.0}
            />
    }
}
