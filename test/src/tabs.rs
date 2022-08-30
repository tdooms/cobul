use cobul::*;
use yew::*;

#[derive(derive_more::Display, strum::EnumIter, PartialEq, Clone, Copy)]
enum Tab {
    #[display(fmt = "tab 1")]
    Tab1,
    #[display(fmt = "tab 2")]
    Tab2,
    #[display(fmt = "tab 3")]
    Tab3,
}

#[function_component(TabsTester)]
pub fn switch_tester() -> Html {
    let state = use_state(|| Tab::Tab1);

    let cloned = state.clone();
    let click = Callback::from(move |x| cloned.set(x));

    html! {
        <Box>
        <simple::Tabs<Tab> value={*state} click={click.clone()} />
        <simple::Tabs<Tab> value={*state} click={click.clone()} rounded=true />
        <simple::Tabs<Tab> value={*state} click={click.clone()} fullwidth=true toggle=true rounded=true />
        <p> {state.to_string()} </p>
        </Box>
    }
}
