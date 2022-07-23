use base::props::{Color, Size};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub id: String,
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
    pub onchange: Callback<bool>,
}

#[derive(derive_more::Display)]
enum Kind {
    #[display(fmt = "checkbox")]
    Checkbox,
    #[display(fmt = "radio")]
    Radio,
}

fn render(props: &Props, kind: Kind) -> Html {
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
    let onchange = props.onchange.reform(move |_| !checked);

    html! {
        <>
        <input id={props.id.clone()} {class} type={kind.to_string()} {onchange} {checked}/>
        <label for={props.id.clone()}> {props.label.clone()} </label>
        </>
    }
}

#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    render(props, Kind::Checkbox)
}

#[function_component(Radio)]
pub fn radio(props: &Props) -> Html {
    render(props, Kind::Radio)
}
