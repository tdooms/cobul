use yew::*;

use cobul::{Button, Buttons, Color, icons::Solid, Notification, use_model};

#[function_component(ButtonModelTest)]
pub fn button_model() -> Html {
    let model = use_model(|| true);
    let value = model.value();

    html! {
        <>
        <Buttons> <Button {model} text="press me if you dare" icon={Solid::TriangleExclamation} color={Color::Danger} /> </Buttons>
        <Notification> { value } </Notification>
        </>
    }
}
