use std::collections::BTreeMap;
use std::rc::Rc;

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
    let mut content = BTreeMap::new();
    content.insert("Components".into(), vec!["Modal".into()]);
    content.insert("Elements".into(), vec!["Block".into(), "Box".into(), "Button".into(), "Content".into(), "Icon".into(), "Notification".into()]);
    content.insert("Extra".into(), vec!["Checkradio".into(), "Loader".into(), "Slider".into(), "Switch".into(), "Tooltip".into()]);
    content.insert("Form".into(), vec!["General".into(), "Derive".into()]);
    content.insert("Functionality".into(), vec!["Button Model".into(), "Field Help".into(), "Field Size".into(), "Slider Modal".into()]);
    content.insert("Simple".into(), vec!["Dropdown".into(), "Pagination".into(), "Tabs".into()]);

    let content = Rc::new(content);
    let model: Model<(AttrValue, AttrValue)> = use_model(|| ("Components".into(), "Modal".into()));

    let inner = match (model.value.0.as_str(), model.value.1.as_str()) {
        ("Components", "Modal") => html! { <ModalTest />},
        ("Elements", "Block") => html! { <BlockTest />},
        ("Elements", "Box") => html! { <BoxTest />},
        ("Elements", "Button") => html! { <ButtonTest />},
        ("Elements", "Content") => html! { <ContentTest />},
        ("Elements", "Icon") => html! { <IconTest />},
        ("Elements", "Notification") => html! { <NotificationTest />},
        ("Extra", "Checkradio") => html! { <CheckradioTest />},
        ("Extra", "Loader") => html! { <LoaderTest />},
        ("Extra", "Slider") => html! { <SliderTest />},
        ("Extra", "Switch") => html! { <SwitchTest />},
        ("Extra", "Tooltip") => html! { <TooltipTest />},
        ("Form", "General") => html! { <GeneralTest />},
        ("Form", "Derive") => html! { <DeriveTest />},
        ("Simple", "Dropdown") => html! { <DropdownTest />},
        ("Simple", "Pagination") => html! { <PaginationTest />},
        ("Simple", "Tabs") => html! { <TabsTest />},
        ("Functionality", "Button Model") => html! { <ButtonModelTest />},
        ("Functionality", "Field Help") => html! { <FieldHelpTest />},
        ("Functionality", "Field Size") => html! { <FieldSizeTest />},
        ("Functionality", "Slider Modal") => html! { <SliderModalTest />},
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
