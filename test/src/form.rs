use yew::*;

use cobul::*;

#[function_component(FormTester)]
pub fn form_tester() -> Html {
    let model = use_model(|| String::new());
    let help = model.value.is_empty().then_some("Rekwajer".to_string());
    let success = model.value.len() > 3;

    html! {
        <Box>
        <simple::Field {help} {success}>
            <Input key="aargh" {model} />
        </simple::Field>
        </Box>
    }
}
