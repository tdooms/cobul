use yew::*;
use cobul::extra::Slider;
use cobul::{Input, Model};

#[function_component(ModelConstantsTest)]
pub fn model_constants() -> Html {
    html! {
        <>
        <Slider<u32> model=60 range={0..100} step=1 label=true disabled=true />
        <Input model="Not changeable" />
        </>
    }
}