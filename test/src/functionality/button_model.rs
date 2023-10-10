use cobul::{use_model, Button, Notification, Color, Buttons, icons::Solid};
use yew::*;

#[function_component(ButtonModelTest)]
pub fn button_model() -> Html {
    let model = use_model(|| true);
    let value = model.value.clone();

    html! {
        <>
        <Buttons> <Button {model} text="press me if you dare" icon={Solid::TriangleExclamation} color={Color::Danger} /> </Buttons>
        <Notification> { value } </Notification>
        </>
    }
}
