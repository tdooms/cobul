use cobul::{Box, Field, Switch};
use yew::*;

#[function_component(SwitchTester)]
pub fn switch_tester() -> Html {
    let state = use_state(|| false);

    let cloned = state.clone();
    let change = Callback::from(move |x| cloned.set(x));

    html! {
        <Box>
        <Field>
        <Switch label="test switch" {change} checked={*state}/>
        </Field>
        <p> {state.to_string()} </p>
        </Box>
    }
}
