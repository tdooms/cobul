use std::str::FromStr;

use yew::prelude::*;

use hooks::use_value_state;

use crate::props::{Color, InputType, Loading, Rounded, Size, Static};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: FromStr + ToString + PartialEq + 'static> {
    pub oninput: Callback<Result<T, String>>,

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

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
#[function_component(TypedInput)]
pub fn typed_input<T>(props: &Props<T>) -> Html
where
    T: FromStr + ToString + PartialEq + 'static,
{
    let state = use_value_state(
        &props
            .value
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
    );

    let classes = classes!(
        "input",
        props.class.clone(),
        props.size,
        props.color,
        props.rounded,
        props.loading,
        props.r#static,
    );

    let oninput = props.oninput.clone();
    let state_clone = state.clone();

    let cb = move |e: InputEvent| {
        let str = e
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();

        oninput.emit(T::from_str(&str).map_err(|_| String::from("error")));
        state_clone.set(str);
    };

    html! {
        <input
            name={props.name.clone()}
            value={(*state).clone()}
            oninput={Callback::from(cb)}
            style={props.style.clone()}
            class={classes}
            type={props.r#type.to_string()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}
