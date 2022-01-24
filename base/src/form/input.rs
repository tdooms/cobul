use yew::prelude::*;

use crate::props::{Color, InputType, Loading, Rounded, Size, Static};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub oninput: Callback<String>,

    #[prop_or_default]
    pub name: Option<String>,

    #[prop_or_default]
    pub value: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub r#type: InputType,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub loading: Loading,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub r#static: Static,
}

/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let classes = classes!(
        "input",
        props.class.clone(),
        props.size,
        props.color,
        props.rounded,
        props.loading,
        props.r#static,
    );

    let oninput = props.oninput.reform(|e: InputEvent| {
        e.target_unchecked_into::<web_sys::HtmlInputElement>()
            .value()
    });

    html! {
        <input
            name={props.name.clone()}
            value={props.value.clone()}
            oninput={oninput}
            class={classes}
            type={props.r#type.to_string()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}
