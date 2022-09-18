use yew::prelude::*;

use crate::props::{Color, FixedSize, Loading, Size, Static};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub name: Option<String>,

    #[prop_or_default]
    pub value: Option<String>,

    #[prop_or_default]
    pub input: Callback<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub placeholder: String,

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
    pub disabled: bool,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub r#static: Static,

    #[prop_or_default]
    pub style: Option<AttrValue>,
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
        props.r#static,
        props.fixed
    );
    let input = props.input.reform(|e: InputEvent| {
        e.target_unchecked_into::<web_sys::HtmlInputElement>()
            .value()
    });

    html! {
        <textarea
            name={props.name.clone()}
            value={props.value.clone()}
            oninput={input}
            style={props.style.clone()}
            {class}
            rows={props.rows.as_ref().map(ToString::to_string)}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}
