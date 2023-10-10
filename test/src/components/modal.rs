use yew::*;
use cobul::{Box, Modal, use_model, Button, Buttons, Color};

#[function_component(ModalTest)]
pub fn modal() -> Html {
    let model1 = use_model(|| false);
    let model2 = use_model(|| false);

    let modal1 = html! {
        <Modal model={model1.clone()}>
        <Box>
        <p> {"This is a very nice modal"} </p>
        <Buttons> <Button text="Leave" model={model1.clone()}/> </Buttons>
        </Box>
        </Modal>
    };

    let footer = html! {
        <Buttons> <Button text="Leave" model={model2.clone()} /> </Buttons>
    };


    let modal2 = html! {
        <Modal model={model2.clone()} {footer} title={"With an even nicer title"}>
            {"This is a very nice modal"}
        </Modal>
    };

    html! {
        <>
        <Buttons>
            <Button text="Normal Modal" color={Color::Light} model={model1} />
            <Button text="Card Modal" color={Color::Light} model={model2} />
        </Buttons>
        {modal1}
        {modal2}
        </>
    }
}