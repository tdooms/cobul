use cobul::{Field, Switch};
use yew::*;

#[function_component(SwitchTester)]
pub fn switch_tester() -> Html {
    let state = use_state(|| false);

    let cloned = state.clone();
    let onchange = Callback::from(move |x| {
        log::info!("changed");
        cloned.set(x)
    });

    html! {
        <>
        <Field>
        <Switch label="test switch" {onchange} id="1" checked={*state}/>
        </Field>
        <p> {state.to_string()} </p>
        </>
    }
}
