use cobul::simple::Pagination;
use yew::*;

#[function_component(SwitchTester)]
pub fn switch_tester() -> Html {
    let state = use_state(|| 0);

    html! {
        <Box>
        <Pagination page={*state} total={10}/>
        </Box>
    }
}
