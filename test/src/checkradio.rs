use cobul::{Box, Checkbox, Color, Field, Radio};
use yew::*;

#[function_component(CheckradioTester)]
pub fn checkradio_tester() -> Html {
    let state = use_state(|| false);

    let cloned = state.clone();
    let input = Callback::from(move |x| cloned.set(x));

    html! {
        <Box>
        <Field>
        <Checkbox label="check0" color={Color::Primary} checked={*state} input={input.clone()}/>
        <Checkbox label="check1" color={Color::Danger} circle=true checked={*state} input={input.clone()}/>
        </Field>
        <Field>
        <Radio label="check2" checked={*state} input={input.clone()}/>
        <Radio label="check3" color={Color::Link} circle=true checked={*state} input={input.clone()}/>
        </Field>
        <p> {state.to_string()} </p>
        </Box>
    }
}
