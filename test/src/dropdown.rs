use yew::*;

use cobul::*;

#[derive(Debug, Clone, PartialEq, Copy, Default, strum::EnumIter, derive_more::Display)]
pub enum TestEnum {
    #[default]
    #[display(fmt = "Option 1")]
    Option1,
    #[display(fmt = "Option 2")]
    Option2,
    #[display(fmt = "Option 3")]
    Option3,
}

#[function_component(DropdownTester)]
pub fn dropdown_tester() -> Html {
    let model = use_model(|| TestEnum::Option1);

    html! {
        <Box>
        <Dropdown<TestEnum> model={model.clone()} />

        <Columns>
        <Column> <Dropdown<TestEnum> model={model.clone()} fullwidth=true /> </Column>
        <Column size={ColumnSize::Is7} />
        </Columns>
        </Box>
    }
}
