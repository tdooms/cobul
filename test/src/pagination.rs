use yew::*;

use cobul::simple::Pagination;
use cobul::Box;

#[function_component(PaginationTester)]
pub fn pagination_tester() -> Html {
    let state = use_state(|| 1);
    let input = ywt::callback!(state; move |idx| state.set(idx));

    html! {
        <Box>
        <Pagination page={*state} total={10} {input}/>
        </Box>
    }
}
