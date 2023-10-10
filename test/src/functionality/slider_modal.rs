use cobul::{Model, use_model, Modal, Button, Buttons, Box};
use cobul::form::{Field, Input};
use yew::*;
use cobul::extra::Slider;

#[function_component(SliderModalTest)]
pub fn slider_modal() -> Html {
    let slider_model = use_model(|| 0);
    let modal_model = use_model(|| false);

    html! {
        <>
        <Buttons>
            <Button model={modal_model.clone()} text="press me" />
        </Buttons>
        <Slider<u32> model={slider_model} range={0..100} step=1 label=true />
        <Modal model={modal_model.clone()} style="width: 600px">
            <Box style="height:300px">
            <Button model={modal_model} text="close"/>
            </Box>
        </Modal>
        </>
    }
}
