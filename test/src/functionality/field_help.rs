use yew::*;

use cobul::use_model;
use cobul::form::{Field, Input};

#[function_component(FieldHelpTest)]
pub fn field_help() -> Html {
    let model = use_model(|| String::new());
    let help = model.value.is_empty().then_some("Rekwajer".to_string());

    html! {
        <Field {help}> <Input key="aargh" {model} /> </Field>
    }
}
