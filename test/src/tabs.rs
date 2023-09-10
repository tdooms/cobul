use yew::*;

use cobul::*;
use cobul::icons::Brands;

#[derive(derive_more::Display, strum::EnumIter, PartialEq, Clone, Copy)]
enum Tab {
    #[display(fmt = "tab 1")]
    Tab1,
    #[display(fmt = "tab 2")]
    Tab2,
    #[display(fmt = "tab 3")]
    Tab3,
}

impl HasIcon for Tab {
    fn icon(&self) -> Option<String> {
        match self {
            Tab::Tab1 => Some(Brands::Medium.to_string()),
            Tab::Tab2 => Some(Brands::Github.to_string()),
            Tab::Tab3 => Some(Brands::Amazon.to_string()),
        }
    }
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
