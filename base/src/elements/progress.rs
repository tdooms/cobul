use yew::prelude::*;

use crate::props::{Color, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_else(|| 1.0)]
    pub max: f32,

    #[prop_or_default]
    pub value: Option<f32>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/elements/progress/](https://bulma.io/documentation/elements/progress/)
#[function_component(Progress)]
pub fn progress(props: &Props) -> Html {
    let classes = classes!("progress", props.class.clone(), props.size, props.color);

    let max = props.max.to_string();
    let value = props.value.as_ref().map(ToString::to_string);

    html! {
        <progress style={props.style.clone()} class={classes} max={max} value={value}>
            // { format!("{}%", self.props.value) }
        </progress>
    }
}
