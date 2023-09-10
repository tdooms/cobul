use yew::prelude::*;

use cobul_props::{Color, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| 1.0)]
    pub max: f32,

    #[prop_or_default]
    pub value: Option<f32>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

/// Native HTML progress bars - [reference](https://bulma.io/documentation/elements/progress/)
///
/// Properties:
/// - `max: f32` &npbs; default: 1.0
/// - `value: Option<f32>`
/// - `color: Option<Color>`
/// - `size: Option<Size>`
#[function_component(Progress)]
pub fn progress(props: &Props) -> Html {
    let class = classes!("progress", props.class.clone(), props.size, props.color);

    let max = props.max.to_string();
    let value = props.value.as_ref().map(ToString::to_string);

    html! {
        <progress style={props.style.clone()} {class} max={max} value={value}>
            // { format!("{}%", self.props.value) }
        </progress>
    }
}
