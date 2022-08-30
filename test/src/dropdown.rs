use cobul::*;
use yew::*;

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

    let change = Callback::from(move |x| state.set(x));

    html! {
        <Box>
        <simple::Dropdown<TestEnum> value={value.clone()} change={change.clone()}/>

        <Columns>
        <Column><simple::Dropdown<TestEnum> {value} {change} fullwidth=true/></Column>
        <Column size={ColumnSize::Is7} />
        </Columns>
        </Box>
    }
}
