use rand::Rng;
use yew::*;

use cobul_props::{Color, Size};
use cobul_model::Model;
use cobul_props::general::{Disabled, Outlined, Rounded, Rtl, Thin};

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub model: Model<bool>,

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

/// Display the classic checkbox as a switch button with different colors, sizes, and states - [reference](https://wikiki.github.io/form/switch/)
#[function_component(Switch)]
pub fn switch(props: &Props) -> Html {
    let id = use_state(|| rand::thread_rng().gen::<u32>().to_string());

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

    let checked = props.model.value();
    let onchange = props.model.toggle().reform(|_| ());
    let disabled = props.disabled.0 || props.model.is_constant();

    html! {
        <>
        <input id={(*id).clone()} {class} type="checkbox" {checked} {disabled} {onchange} />
        <label for={(*id).clone()}> {props.label.clone()} </label>
        </>
    }
}
