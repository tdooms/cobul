use yew::*;

use cobul::{use_model, Box, Color, Field};
use cobul::extra::{Checkbox, Radio};

#[function_component(CheckradioTester)]
pub fn checkradio_tester() -> Html {
    let model = use_model(|| false);

    html! {
        <Box>
        <Field>
        <Checkbox label="check0" model={model.clone()} color={Color::Primary} />
        <Checkbox label="check1" model={model.clone()} color={Color::Danger} circle=true/>
        </Field>
        <Field>
        <Radio label="check2" model={model.clone()} />
        <Radio label="check3" model={model.clone()} color={Color::Link} circle=true />
        </Field>
        <p> {model.value.to_string()} </p>
        </Box>
    }
}
