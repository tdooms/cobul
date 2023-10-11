use yew::*;

use cobul::use_model;
use cobul::simple::Pagination;

#[function_component(PaginationTest)]
pub fn pagination() -> Html {
    let model = use_model(|| 1);

    html! {
        <Pagination {model} total={10} />
    }
}
