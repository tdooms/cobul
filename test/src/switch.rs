use cobul::{Box, Field, Switch};
use yew::*;

#[function_component(SwitchTester)]
pub fn switch_tester() -> Html {
    let state = use_state(|| false);

    let cloned = state.clone();
    let input = Callback::from(move |x| cloned.set(x));

    html! {
        <Box>
        <Field>
        <Switch label="test switch" {input} value={*state}/>
        </Field>
        <p> {state.to_string()} </p>
        </Box>
    }
}
