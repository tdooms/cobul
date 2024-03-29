use web_sys::HtmlInputElement;
use yew::prelude::*;

use cobul_props::{Color, Size};
use cobul_model::Model;
use cobul_props::general::{Disabled, FixedSize, Loading, Readonly, Static};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub model: Model<AttrValue>,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    #[prop_or_default]
    pub name: Option<AttrValue>,

    #[prop_or_default]
    pub rows: Option<u32>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub fixed: FixedSize,

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

/// The multiline textarea and its variations - [reference](https://bulma.io/documentation/form/textarea/)
///
/// Properties:
/// - `model: Model<String>`
/// - `placeholder: Option<AttrValue>`
/// - `name: Option<AttrValue>`
/// - `rows: Option<u32>`
/// - `size: Option<Size>`
/// - `color: Option<Color>`
/// - `fixed: FixedSize`
/// - `loading: Loading`
/// - `disabled: Disabled`
/// - `readonly: Readonly`
/// - `statik: Static`
#[function_component(Textarea)]
pub fn textarea(props: &Props) -> Html {
    let size = use_context::<Size>();
    let color = use_context::<Color>();
    let loading = use_context::<Loading>();
    let statik = use_context::<Static>();
    let fixed = use_context::<FixedSize>();

    let class = classes!(
        "textarea",
        props.class.clone(),
        props.color.or(color),
        props.size.or(size),
        props.loading.or(loading),
        props.statik.or(statik),
        props.fixed.or(fixed)
    );

    let reform = |e: InputEvent| e.target_unchecked_into::<HtmlInputElement>().value().into();
    let oninput = props.model.reform(reform);

    html! {
        <textarea
            value={props.model.value()}
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
