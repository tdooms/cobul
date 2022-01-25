use yew::prelude::*;

use crate::props::{Color, InputType, Loading, Rounded, Size, Static};
use std::str::FromStr;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: FromStr + ToString + PartialEq + 'static> {
    pub oninput: Callback<T>,

    #[prop_or_default]
    pub name: Option<String>,

    #[prop_or_default]
    pub value: Option<T>,

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
#[function_component(TypedInput)]
pub fn typed_input<T: FromStr + ToString + PartialEq + 'static>(props: &Props<T>) -> Html {
    let classes = classes!(
        "input",
        props.class.clone(),
        props.size,
        props.color,
        props.rounded,
        props.loading,
        props.r#static,
    );

    let cloned = props.oninput.clone();

    let oninput = Callback::from(move |e: InputEvent| {
        let str = e
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();
        match T::from_str(&str) {
            Ok(value) => cloned.emit(value),
            Err(_) => {}
        }
    });

    html! {
        <input
            name={props.name.clone()}
            value={props.value.as_ref().map(ToString::to_string)}
            oninput={oninput}
            class={classes}
            type={props.r#type.to_string()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}
