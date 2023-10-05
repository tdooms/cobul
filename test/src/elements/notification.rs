use cobul::{use_model, Box, Notification};
use yew::*;

#[function_component(NotificationTest)]
pub fn notification() -> Html {
    let model = use_model(|| true);

    let value = model.value.clone();
    let close = model.input.reform(move |_| !value);

    html! {
        <>
        <Notification> {"Hello world!"} </Notification>
        <Notification delete={close}> {"Hello world!"} </Notification>

        <Box> {value} </Box>
        </>
    }
}
