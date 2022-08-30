use base::props::{Color, Size};
use rand::Rng;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub label: String,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub circle: bool,

    #[prop_or(true)]
    pub border: bool,

    #[prop_or_default]
    pub background: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub rtl: bool,

    #[prop_or_default]
    pub block: Option<bool>,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub input: Callback<bool>,
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
        (!props.border).then(|| "has-no-border"),
        props.background.then(|| "has-background-color"),
        props.rtl.then(|| "is-rtl"),
    );

    let checked = props.checked;
    let input = props.input.reform(move |_| !checked);

    html! {
        <>
        <input id={id.clone()} {class} type={kind.to_string()} onchange={input} {checked}/>
        <label for={id}> {props.label.clone()} </label>
        </>
    }
}

#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let id = use_state(|| rand::thread_rng().gen::<u64>().to_string());
    render(props, Kind::Checkbox, (*id).clone())
}

#[function_component(Radio)]
pub fn radio(props: &Props) -> Html {
    let id = use_state(|| rand::thread_rng().gen::<u64>().to_string());
    render(props, Kind::Radio, (*id).clone())
}
