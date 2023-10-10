use std::fmt::{Debug, Display};
use std::ops::Range;

use num::{FromPrimitive, ToPrimitive};
use rand::Rng;
use web_sys::{HtmlDivElement, HtmlInputElement, HtmlOutputElement};
use yew::prelude::*;

use cobul_props::{Color, Model, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: PartialEq + Clone> {
    #[prop_or_default]
    pub change: Callback<T>, // change is triggered when the slider is released (deferred)

    #[prop_or_default]
    pub input: Callback<T>, // input is triggered when the slider is moved (instant)

    #[prop_or_default]
    pub model: Option<Model<T>>, // model uses the oninput property (instant)

    #[prop_or_default]
    pub value: Option<T>,

    pub step: T,

    pub range: Range<T>,

    #[prop_or("{}")]
    pub fmt: &'static str,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub fullwidth: bool,

    #[prop_or_default]
    pub size: Option<Size>,

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
    pub label_width: f64,
}

/// Display a classic slider with different colors, sizes, and states - [reference](https://wikiki.github.io/form/slider/)
#[function_component(Slider)]
pub fn slider<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Clone + Display + FromPrimitive + ToPrimitive + 'static,
{
    let id = use_state(|| rand::thread_rng().gen::<u32>().to_string());

    if props.label && props.tooltip {
        panic!("both right label and tooltip are not supported");
    }

    let width = use_state(|| 0);
    let container = use_node_ref();
    let label = use_node_ref();

    let cloned = width.clone();
    use_effect_with(
        (container.clone(), label.clone()),
        move |(container, label)| {
            if let (Some(container), Some(label)) = (
                container.cast::<HtmlDivElement>(),
                label.cast::<HtmlOutputElement>(),
            ) {
                cloned.set(container.offset_width() - label.offset_width())
            }
        },
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
        props.label.then(|| "my-0")
    );

    let (value, input) = Model::split(&props.model);

    let onchange = props.change.reform(|e: Event| {
        let elem = e.target_unchecked_into::<HtmlInputElement>();
        T::from_f64(elem.value_as_number()).unwrap()
    });
    let oninput = input.reform(|e: InputEvent| {
        let elem = e.target_unchecked_into::<HtmlInputElement>();
        T::from_f64(elem.value_as_number()).unwrap()
    });

    let formatted = props.fmt.replace("{}", &value.clone().unwrap().to_string());

    let (start, end, value) = (
        props.range.start.to_f64().unwrap(),
        props.range.end.to_f64().unwrap(),
        value.clone().unwrap().to_f64().unwrap(),
    );

    let offset = *width as f64 * ((value - start) / (end - start)).clamp(0.0, 1.0);

    let output_style = match props.tooltip {
        true => format!("left:{offset}px;width:{}rem", props.label_width),
        false => format!("width:{}rem;top:0px ! important", props.label_width),
    };

    let input_style = match props.tooltip {
        false => format!("width:calc(100% - ({}rem))", props.label_width + 1.2),
        true => "width:100%".to_owned(),
    };

    let (min, max) = (props.range.start.to_string(), props.range.end.to_string());

    html! {
        <div style="position:relative" ref={container} class="pt-2">
        <input style={input_style} id={(*id).clone()} {class} step={props.step.to_string()} {min} {max}
                disabled={props.disabled} orient={props.vertical.then(|| "vertical")} {onchange} {oninput} type="range" value={value.to_string()}/>
        <output style={output_style} for={(*id).clone()} ref={label}> {formatted} </output>
        </div>
    }
}
