use base::model::Model;
use rand::Rng;
use yew::*;

use base::props::{Color, Disabled, Outlined, Rounded, Rtl, Size, Thin};
use base::utils::combine_model;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub value: Option<bool>,

    #[prop_or_default]
    pub input: Callback<bool>,

    #[prop_or_default]
    pub model: Option<Model<bool>>,

    #[prop_or_else(|| "Label".into())]
    pub label: AttrValue,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub rtl: Rtl,

    #[prop_or_default]
    pub thin: Thin,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub outlined: Outlined,

    #[prop_or_default]
    pub disabled: Disabled,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Switch)]
pub fn switch(props: &Props) -> Html {
    let id = use_state(|| rand::thread_rng().gen::<u64>().to_string());

    let class = classes!(
        "switch",
        props.class.clone(),
        props.rtl,
        props.color,
        props.size,
        props.thin,
        props.rounded,
        props.outlined,
    );

    let (input, value) = combine_model(&props.input, &props.value, &props.model);
    let checked = value.unwrap_or_default();
    let onchange = input.reform(move |_| !checked);

    html! {
        <>
        <input id={(*id).clone()} {class} type="checkbox" {checked} disabled={props.disabled.0} {onchange} />
        <label for={(*id).clone()}> {props.label.clone()} </label>
        </>
    }
}
