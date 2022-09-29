use yew::*;

use cobul::{use_model, Box, Field, Switch};

#[function_component(SwitchTester)]
pub fn switch_tester() -> Html {
    let model = use_model(|| false);

    html! {
        <Box>
        <Field>
        <Switch label="test switch" model={model.clone()} />
        </Field>
        <p> {model.value.to_string()} </p>
        </Box>
    }
}
