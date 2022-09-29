use yew::*;

use crate::use_model;
use cobul::{Box, Slider};

#[function_component(SliderTester)]
pub fn slider_tester() -> Html {
    let model = use_model(|| 50.0);

    let floats = html! {
        <>
        <Slider<f32> model={model.clone()} range={0.0..100.0} step=1.0 fullwidth=true label=true fmt="{}%" label_width=3.5 />
        <Slider<f32> model={model.clone()} range={0.0..50.0} step=1.0 fullwidth=true tooltip=true/>
        <p> {model.value.to_string()} </p>
        </>
    };

    let model = use_model(|| 50);

    let integers = html! {
        <>
        <Slider<u32> model={model.clone()} range={0..100} step=1 fullwidth=true tooltip=true/>
        <Slider<u32> model={model.clone()} range={0..20} step=1 fullwidth=true label=true fmt="{}px" label_width=3.6 />
        <p> {model.value.to_string()} </p>
        </>
    };

    html! {
        <Box>
        {floats}
        {integers}
        </Box>
    }
}
