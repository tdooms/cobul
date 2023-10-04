use yew::*;
use cobul::{use_model, Button};

#[function_component(ButtonModelTest)]
pub fn button_model() -> Html {
    let model = use_model(|| true);
    let value = model.value.clone();

    html! {
        <>
        <Button {model} text="press meee" />
        { value }
        </>
    }
}