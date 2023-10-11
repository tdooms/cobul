use yew::*;
use cobul::*;

#[function_component(FieldHelpTest)]
pub fn field_help() -> Html {
    let model = use_model(AttrValue::default);
    let help = model.value.is_empty().then_some("Rekwajered".to_string());

    html! {
        <>
        <Field help={help.clone()}> <Input model={model.clone()} /> </Field>
        <Field help={help.clone()} color={Color::Danger}> <Input model={model.clone()} /> </Field>
        <Field help={help.clone()} color={Color::Warning}> <Input model={model.clone()} /> </Field>
        </>
    }
}
