use cobul_props::general::{Disabled, Loading, Readonly, Rounded, Static};
use cobul_props::{Color, Model, Size};
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

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

/// The text input and its variations - [reference](https://bulma.io/documentation/form/input/)
///     
/// Properties:
/// - `input: Callback<String>` Callback for when the input is changed
/// - `value: Option<String>`
/// - `model: Option<Model<String>>`
/// - `name: Option<AttrValue>`
/// - `kind: Option<AttrValue>`
/// - `placeholder: Option<AttrValue>`
/// - `size: Option<Size>`
/// - `color: Option<Color>`
/// - `rounded: Rounded`
/// - `loading: Loading`
/// - `disabled: Disabled`
/// - `readonly: Readonly`
/// - `statik: Static`
#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let class = classes!(
        "input",
        props.class.clone(),
        props.size,
        props.color,
        props.rounded,
        props.loading,
        props.statik,
    );

    let (input, value) = Model::combine(&props.input, &props.value, &props.model);
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
