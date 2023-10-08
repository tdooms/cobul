use yew::*;

use cobul::*;

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
    let model = use_model(|| Tab::Tab1);

    html! {
        <Box>
        <Tabs<Tab> model={model.clone()} />
        <Tabs<Tab> model={model.clone()} rounded=true />
        <Tabs<Tab> model={model.clone()} fullwidth=true toggle=true rounded=true />
        <p> {model.value.to_string()} </p>
        </Box>
    }
}
