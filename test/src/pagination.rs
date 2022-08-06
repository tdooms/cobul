use cobul::simple::Pagination;
use cobul::Box;
use yew::*;

#[function_component(PaginationTester)]
pub fn pagination_tester() -> Html {
    let state = use_state(|| 1);
    let onchange = ywt::callback!(state; move |idx| state.set(idx));

    html! {
        <Box>
        <Pagination page={*state} total={10} {onchange}/>
        </Box>
    }
}
