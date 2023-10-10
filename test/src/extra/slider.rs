use yew::*;

use crate::use_model;
use cobul::{Box, extra::Slider, Notification};

#[function_component(SliderTest)]
pub fn slider() -> Html {
    let model = use_model(|| 50.0);

    let floats = html! {
        <>
        <Slider<f32> model={model.clone()} range={0.0..50.0} step=1.0 fullwidth=true tooltip=true/>
        <Slider<f32> model={model.clone()} range={0.0..100.0} step=1.0 fullwidth=true label=true fmt="{}%" label_width=3.5 />
        <Notification> {model.value.to_string()} {"%"} </Notification>
        </>
    };

    let model = use_model(|| 50);

    let integers = html! {
        <>
        <Slider<u32> model={model.clone()} range={0..100} step=1 fullwidth=true tooltip=true/>
        <Slider<u32> model={model.clone()} range={0..20} step=1 fullwidth=true label=true fmt="{}px" label_width=3.6 />
        <Notification> {model.value.to_string()} {"px"} </Notification>
        </>
    };

    html! {
        <Box>
        {floats}
        {integers}
        </Box>
    }
}
