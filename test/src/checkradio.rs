use cobul::{Box, Checkbox, Color, Field, Radio};
use yew::*;

#[function_component(CheckradioTester)]
pub fn checkradio_tester() -> Html {
    let state = use_state(|| false);

    let cloned = state.clone();
    let onchange = Callback::from(move |x| cloned.set(x));

    html! {
        <Box>
        <Field>
        <Checkbox id="check0" label="check0" color={Color::Primary} checked={*state} onchange={onchange.clone()}/>
        <Checkbox id="check1" label="check1" color={Color::Danger} circle=true checked={*state} onchange={onchange.clone()}/>
        </Field>
        <Field>
        <Radio id="check2" label="check2" checked={*state} onchange={onchange.clone()}/>
        <Radio id="check3" label="check3" color={Color::Link} circle=true checked={*state} onchange={onchange.clone()}/>
        </Field>
        <p> {state.to_string()} </p>
        </Box>
    }
}
