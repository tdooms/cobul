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
    let state = use_state(|| TestEnum::Option1);
    let value = *state;

    let input = Callback::from(move |x| state.set(x));

    html! {
        <Box>
        <simple::Dropdown<TestEnum> value={value.clone()} input={input.clone()}/>

        <Columns>
        <Column><simple::Dropdown<TestEnum> {value} {input} fullwidth=true/></Column>
        <Column size={ColumnSize::Is7} />
        </Columns>
        </Box>
    }
}
