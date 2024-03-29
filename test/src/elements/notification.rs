use yew::*;

use cobul::{Color, Notification, use_model};

#[function_component(NotificationTest)]
pub fn notification() -> Html {
    let model = use_model(|| false);

    let value = model.value().clone();
    let close = model.reform(move |_| true);

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
