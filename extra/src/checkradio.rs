use rand::Rng;
use yew::prelude::*;

use cobul_props::{Color, Size};
use cobul_model::Model;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub model: Model<bool>,

    #[prop_or_default]
    pub label: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub circle: bool,

    #[prop_or_default]
    pub block: bool,

    #[prop_or(true)]
    pub border: bool,

    #[prop_or_default]
    pub background: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub rtl: bool,
}

#[derive(derive_more::Display)]
enum Kind {
    #[display(fmt = "checkbox")]
    Checkbox,
    #[display(fmt = "radio")]
    Radio,
}

fn render(props: &Props, kind: Kind, id: String) -> Html {
    let class = classes!(
        "is-checkradio",
        props.class.clone(),
        props.color.clone(),
        props.size.clone(),
        props.circle.then(|| "is-circle"),
        props.block.then(|| "is-block"),
        (!props.border).then(|| "has-no-border"),
        props.background.then(|| "has-background-color"),
        props.rtl.then(|| "is-rtl"),
    );

    let checked = props.model.value();
    let onchange = props.model.toggle().reform(|_| ());
    let disabled = props.disabled || props.model.is_constant();

    html! {
        <>
        <input id={id.clone()} {class} type={kind.to_string()} {onchange} {checked} {disabled} />
        <label for={id}> {props.label.clone()} </label>
        </>
    }
}

///  Make classic checkbox and radio sexier with different colors, sizes, and states  - [reference](https://wikiki.github.io/form/checkradio/)
#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let id = use_state(|| rand::thread_rng().gen::<u32>().to_string());
    render(props, Kind::Checkbox, (*id).clone())
}

///  Make classic checkbox and radio sexier with different colors, sizes, and states  - [reference](https://wikiki.github.io/form/checkradio/)
#[function_component(Radio)]
pub fn radio(props: &Props) -> Html {
    let id = use_state(|| rand::thread_rng().gen::<u32>().to_string());
    render(props, Kind::Radio, (*id).clone())
}
