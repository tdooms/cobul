use cobul::*;
use yew::*;

#[function_component(FormHelp)]
pub fn form_help() -> Html {
    let model = use_model(|| String::new());
    let help = model.value.is_empty().then_some("Rekwajer".to_string());

    html! {
        <Box>
        <Field {help}> <Input key="aargh" {model} /> </Field>
        </Box>
    }
}
