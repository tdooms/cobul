use yew::*;
use cobul::*;

#[derive(Debug, Clone, PartialEq, Copy, strum::EnumIter, derive_more::Display)]
pub enum TestEnum {
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

    let onchange = Callback::from(move |x| state.set(x));

    html! {
        <Box>
        <simple::Dropdown<TestEnum> {value} {onchange}/>
        </Box>
    }
}