use yew::*;

use cobul::Pagination;
use cobul::{use_model, Box};

#[function_component(PaginationTester)]
pub fn pagination_tester() -> Html {
    let model = use_model(|| 1);

    html! {
        <Box>
        <Pagination {model} total={10} />
        </Box>
    }
}
