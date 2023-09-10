use yew::*;

use cobul::*;

#[function_component(FormTester)]
pub fn form_tester() -> Html {
    let model = use_model(|| String::new());
    let help = model.value.is_empty().then_some("Rekwajer".to_string());

    html! {
        <Box>
        <Field {help}> <Input key="aargh" {model} /> </Field>
        </Box>
    }
}
