use num::{FromPrimitive, ToPrimitive};
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::str::FromStr;
use web_sys::{HtmlDivElement, HtmlOutputElement};
use yew::prelude::*;

use base::props::{Color, Fullwidth, Size};

#[derive(Clone, Debug, PartialEq)]
pub enum LabelAlignment {
    Left,
    Right,
    Tooltip,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: PartialEq> {
    pub id: &'static str,

    pub step: T,
    pub range: Range<T>,

    pub value: T,

    /// Onchange is triggered when the slider is released
    #[prop_or_default]
    pub onchange: Callback<T>,

    // Oninput is triggered when the slider is moved (instantly)
    #[prop_or_default]
    pub oninput: Callback<T>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub fullwidth: Fullwidth,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub circle: bool,

    #[prop_or_default]
    pub vertical: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub label: Option<LabelAlignment>,
}

/// [https://wikiki.github.io/form/slider/](https://wikiki.github.io/form/slider/)
#[function_component(Slider)]
pub fn slider<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Clone + Display + FromPrimitive + ToPrimitive + 'static,
{
    let width = use_state(|| 0);
    let container = use_node_ref();
    let label = use_node_ref();

    let cloned = width.clone();
    use_effect_with_deps(
        move |(container, label)| {
            if let (Some(container), Some(label)) = (
                container.cast::<HtmlDivElement>(),
                label.cast::<HtmlOutputElement>(),
            ) {
                cloned.set(container.offset_width() - label.offset_width())
            }
            || ()
        },
        (container.clone(), label.clone()),
    );

    let offset = *width as f64 * props.value.to_f64().unwrap() / 100.0;

    let output = match props.label {
        None => "",
        Some(LabelAlignment::Left | LabelAlignment::Right) => "has-output",
        Some(LabelAlignment::Tooltip) => "has-output-tooltip",
    };

    let class = classes!(
        "slider",
        props.class.clone(),
        props.size,
        props.color,
        props.fullwidth,
        props.circle.then(|| "is-circle"),
        output
    );

    let onchange = props.onchange.reform(|e: Event| {
        let elem = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        T::from_f64(elem.value_as_number()).unwrap()
    });
    let oninput = props.oninput.reform(|e: InputEvent| {
        let elem = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        T::from_f64(elem.value_as_number()).unwrap()
    });

    let (pre, post) = match props.label {
        Some(LabelAlignment::Left) => (
            html! {<output for={props.id}> {props.value.clone()} </output>},
            html! {},
        ),
        Some(LabelAlignment::Right) => (
            html! {},
            html! {<output for={props.id}> {props.value.clone()} </output>},
        ),
        Some(LabelAlignment::Tooltip) => (
            html! {},
            html! {<output style={format!("left:{offset}px")} for={props.id} ref={label}> {props.value.clone()} </output>},
        ),
        None => (html! {}, html! {}),
    };

    html! {
        <div style="position:relative" ref={container} class="pt-2 m-2">
        {pre}
        <input id={props.id} {class} step={props.step.to_string()} min={props.range.start.to_string()} max={props.range.end.to_string()}
                disabled={props.disabled} orient={props.vertical.then(|| "vertical")} {onchange} {oninput} type="range"/>
        {post}
        </div>
    }
}
