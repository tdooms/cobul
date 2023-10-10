use cobul::{use_model, Notification, Color};
use yew::*;

#[function_component(NotificationTest)]
pub fn notification() -> Html {
    let model = use_model(|| false);

    let value = model.value.clone();
    let close = model.input.reform(move |_| true);

    html! {
        <>
        <Notification color={Color::Danger}> {"Something horrible is going to happen!"} </Notification>
        <Notification color={Color::Success}> {"You did very well!"} </Notification>
        <Notification color={Color::Success} light=true> {"Light color woo!"} </Notification>
        <Notification delete={close}> {"Close me please!"} </Notification>

        <Notification> {"Clicked close: "} <b>{value}</b> </Notification>
        </>
    }
}
