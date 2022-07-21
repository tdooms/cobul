use cobul::{Field, Switch, Box};
use yew::*;

#[function_component(SwitchTester)]
pub fn switch_tester() -> Html {
    let state = use_state(|| false);

    let cloned = state.clone();
    let onchange = Callback::from(move |x| cloned.set(x));

    html! {
        <Box>
        <Field>
        <Switch label="test switch" {onchange} id="1" checked={*state}/>
        </Field>
        <p> {state.to_string()} </p>
        </Box>
    }
}
