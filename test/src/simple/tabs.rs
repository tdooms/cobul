use yew::*;

use cobul::*;
use cobul::simple::Tabs;

#[derive(derive_more::Display, strum::EnumIter, PartialEq, Clone, Copy)]
enum Tab {
    #[display(fmt = "tab 1")]
    Tab1,
    #[display(fmt = "tab 2")]
    Tab2,
    #[display(fmt = "tab 3")]
    Tab3,
}

#[function_component(TabsTest)]
pub fn switch() -> Html {
    let model = use_model(|| Tab::Tab1);

    html! {
        <>
        <Tabs<Tab> model={model.clone()} />
        <Tabs<Tab> model={model.clone()} rounded=true />
        <Tabs<Tab> model={model.clone()} fullwidth=true toggle=true rounded=true />
        <Notification> {model.value.to_string()} </Notification>
        </>
    }
}
