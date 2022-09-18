use std::str::FromStr;
use yew::prelude::*;

use crate::props::{Color, InputType, Loading, Rounded, Size, Static};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: FromStr + ToString + PartialEq + 'static> {
    #[prop_or_default]
    pub input: Callback<Result<T, String>>,

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
    pub size: Option<Size>,

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
    pub style: Option<AttrValue>,
}

/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
#[function_component(TypedInput)]
pub fn typed_input<T>(props: &Props<T>) -> Html
where
    T: FromStr + ToString + PartialEq + 'static,
{
    let value = props
        .value
        .as_ref()
        .map(|x| x.to_string())
        .unwrap_or_default();
    let cloned = value.clone();

    let state = use_state_eq(move || cloned);

    let cloned = state.clone();
    use_effect_with_deps(
        move |value| {
            cloned.set(value.clone());
            || ()
        },
        value,
    );

    let class = classes!(
        "input",
        props.class.clone(),
        props.size,
        props.color,
        props.rounded,
        props.loading,
        props.r#static,
    );

    let input = props.input.clone();
    let state_clone = state.clone();

    let cb = move |e: InputEvent| {
        let str = e
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .value();

        input.emit(T::from_str(&str).map_err(|_| String::from("error")));
        state_clone.set(str);
    };

    html! {
        <input
            name={props.name.clone()}
            value={(*state).clone()}
            oninput={Callback::from(cb)}
            style={props.style.clone()}
            {class}
            type={props.r#type.to_string()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}
