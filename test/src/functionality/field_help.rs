use yew::*;

use cobul::{use_model, Field, Input};

#[function_component(FieldHelpTest)]
pub fn field_help() -> Html {
    let model = use_model(AttrValue::default);
    let help = model.value.is_empty().then_some("Rekwajer".to_string());

    html! {
        <Field {help}> <Input key="aargh" {model} /> </Field>
    }
}
