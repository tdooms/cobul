use yew::*;

use cobul::{extra::Switch, Field, Notification, use_model};

#[function_component(SwitchTest)]
pub fn switch() -> Html {
    let model = use_model(|| false);

    html! {
        <>
        <Field>
        <Switch label="test switch" model={model.clone()} />
        </Field>
        <Notification> {model.value.to_string()} </Notification>
        </>
    }
}
