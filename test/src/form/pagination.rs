use yew::*;

use cobul::form::Pagination;
use cobul::{use_model, Box};

#[function_component(PaginationTest)]
pub fn pagination() -> Html {
    let model = use_model(|| 1);

    html! {
        <Pagination {model} total={10} />
    }
}
