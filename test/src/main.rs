use implicit_clone::unsync::{IString, IMap, IArray};
use yew::prelude::*;
use cobul::{Box, Column, Columns, ColumnSize, Container, Menu, Model, use_model};

use crate::components::*;
use crate::elements::*;
use crate::extra::*;
use crate::form::*;
use crate::simple::*;
use crate::functionality::*;

mod elements;
mod simple;
mod functionality;
mod extra;
mod components;
mod form;


#[function_component(App)]
fn app() -> Html {
    let model: Model<(AttrValue, AttrValue)> = use_model(|| ("Components".into(), "Modal".into()));

    let components: IArray<_> = ["Modal"].into_iter().map(Into::into).collect();
    let elements: IArray<_> = ["Block", "Box", "Button", "Content", "Icon", "Notification"].into_iter().map(Into::into).collect();
    let extra: IArray<_> = ["Check & Radio", "Loader", "Slider", "Switch", "Tooltip"].into_iter().map(Into::into).collect();
    let form: IArray<_> = ["General", "Derive", "Lists", "Partial"].into_iter().map(Into::into).collect();
    let simple: IArray<_> = ["Dropdown", "Pagination", "Tabs"].into_iter().map(Into::into).collect();
    let functionality: IArray<_> = ["Button Model", "Field Help", "Field Size", "Slider Modal", "Model With"].into_iter().map(Into::into).collect();

    let content: IMap<_, _> = [
        ("Components".into(), components),
        ("Elements".into(), elements),
        ("Extra".into(), extra),
        ("Form".into(), form),
        ("Simple".into(), simple),
        ("Functionality".into(), functionality),
    ].into_iter().collect();


    let inner = match (model.value.0.as_str(), model.value.1.as_str()) {
        ("Components", "Modal") => html! { <ModalTest />},
        ("Elements", "Block") => html! { <BlockTest />},
        ("Elements", "Box") => html! { <BoxTest />},
        ("Elements", "Button") => html! { <ButtonTest />},
        ("Elements", "Content") => html! { <ContentTest />},
        ("Elements", "Icon") => html! { <IconTest />},
        ("Elements", "Notification") => html! { <NotificationTest />},
        ("Extra", "Check & Radio") => html! { <CheckRadioTest />},
        ("Extra", "Loader") => html! { <LoaderTest />},
        ("Extra", "Slider") => html! { <SliderTest />},
        ("Extra", "Switch") => html! { <SwitchTest />},
        ("Extra", "Tooltip") => html! { <TooltipTest />},
        ("Form", "General") => html! { <GeneralTest />},
        ("Form", "Derive") => html! { <DeriveTest />},
        ("Form", "Lists") => html! { <ListsTest />},
        ("Form", "Partial") => html! { <PartialTest />},
        ("Simple", "Dropdown") => html! { <DropdownTest />},
        ("Simple", "Pagination") => html! { <PaginationTest />},
        ("Simple", "Tabs") => html! { <TabsTest />},
        ("Functionality", "Button Model") => html! { <ButtonModelTest />},
        ("Functionality", "Field Help") => html! { <FieldHelpTest />},
        ("Functionality", "Field Size") => html! { <FieldSizeTest />},
        ("Functionality", "Slider Modal") => html! { <SliderModalTest />},
        ("Functionality", "Model With") => html! { <ModelWithTest />},
        _ => html! {"not found"}
    };

    html! {
        <Container class="mt-6">
        <Columns>
        <Column size={ColumnSize::Is2}> <Box> <Menu {content} {model} /> </Box> </Column>
        <Column> <Box> {inner} </Box> </Column>
        </Columns>
        </Container>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
