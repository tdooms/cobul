use num::{FromPrimitive, ToPrimitive};
use std::fmt::{Debug, Display};
use std::ops::Range;
use web_sys::{HtmlDivElement, HtmlInputElement, HtmlOutputElement};
use yew::prelude::*;

use base::props::{Color, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: PartialEq> {
    pub id: &'static str,

    pub step: T,
    pub range: Range<T>,

    pub value: T,

    #[prop_or("{}")]
    pub fmt: &'static str,

    /// Onchange is triggered when the slider is released
    #[prop_or_default]
    pub onchange: Callback<T>,

    // Oninput is triggered when the slider is moved (instantly)
    #[prop_or_default]
    pub oninput: Callback<T>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub fullwidth: bool,

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
    pub tooltip: bool,

    #[prop_or_default]
    pub label: bool,

    #[prop_or(3.0)]
    pub labelwidth: f64,
}

/// [https://wikiki.github.io/form/slider/](https://wikiki.github.io/form/slider/)
#[function_component(Slider)]
pub fn slider<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Clone + Display + FromPrimitive + ToPrimitive + 'static,
{
    if props.label && props.tooltip {
        panic!("both right label and tooltip are not supported");
    }

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

    let class = classes!(
        "slider",
        props.class.clone(),
        props.size,
        props.color,
        props.fullwidth.then(|| "is-fullwidth"),
        props.circle.then(|| "is-circle"),
        props.tooltip.then(|| "has-output-tooltip"),
        props.label.then(|| "has-output"),
    );

    let onchange = props.onchange.reform(|e: Event| {
        let elem = e.target_unchecked_into::<HtmlInputElement>();
        T::from_f64(elem.value_as_number()).unwrap()
    });
    let oninput = props.oninput.reform(|e: InputEvent| {
        let elem = e.target_unchecked_into::<HtmlInputElement>();
        T::from_f64(elem.value_as_number()).unwrap()
    });

    let formatted = props.fmt.replace("{}", &props.value.to_string());

    let (start, end, value) = (
        props.range.start.to_f64().unwrap(),
        props.range.end.to_f64().unwrap(),
        props.value.to_f64().unwrap(),
    );

    let offset = *width as f64 * ((value - start) / (end - start)).clamp(0.0, 1.0);

    let output_style = match props.tooltip {
        true => format!(
            "z-index:-100px;left:{offset}px;width:{}rem",
            props.labelwidth
        ),
        false => format!("z-index:-100;width:{}rem", props.labelwidth),
    };

    let input_style = match props.tooltip {
        false => format!("width:calc(100% - ({}rem))", props.labelwidth + 1.2),
        true => "width:100%".to_owned(),
    };

    html! {
        <div style="position:relative" ref={container} class="pt-2">
        <input style={input_style} id={props.id} {class} step={props.step.to_string()} min={props.range.start.to_string()} max={props.range.end.to_string()}
                disabled={props.disabled} orient={props.vertical.then(|| "vertical")} {onchange} {oninput} type="range" value={props.value.to_string()}/>
        <output style={output_style} for={props.id} ref={label}> {formatted} </output>
        </div>
    }
}
