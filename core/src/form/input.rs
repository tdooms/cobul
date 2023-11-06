use web_sys::HtmlInputElement;
use yew::prelude::*;

use cobul_props::{Color, Size};
use cobul_model::Model;
use cobul_props::general::{Disabled, Loading, Readonly, Rounded, Static};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub model: Model<AttrValue>,

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
    let size = props.size.or(use_context::<Option<Size>>().flatten());
    let color = props.color.or(use_context::<Option<Color>>().flatten());
    
    let class = classes!(
        "input",
        props.class.clone(),
        props.rounded,
        props.loading,
        props.statik,
        size,
        color,
    );

    let reform = |e: InputEvent| e.target_unchecked_into::<HtmlInputElement>().value().into();
    let oninput = props.model.reform(reform);
    let disabled = props.disabled.0 || props.model.is_constant();

    html! {
        <input
            value={props.model.value()}
            {class}
            {oninput}
            name={props.name.clone()}
            type={props.kind.clone()}
            style={props.style.clone()}
            placeholder={props.placeholder.clone()}
            {disabled}
            readonly={props.readonly.0}
            />
    }
}
