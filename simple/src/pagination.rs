use yew::*;

use base::components;
use base::components::{PaginationEllipsis, PaginationLink};
use base::model::Model;
use base::props::{Alignment, Size};
use base::utils::combine_model;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub input: Callback<u64>,

    #[prop_or_default]
    pub value: Option<u64>,

    #[prop_or_default]
    pub model: Option<Model<u64>>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub rounded: bool,

    pub total: u64,
}

#[function_component(Pagination)]
pub fn pagination(props: &Props) -> Html {
    let Props {
        size,
        class,
        alignment,
        rounded,
        total,
        ..
    } = props.clone();

    let (input, value) = combine_model(&props.input, &props.value, &props.model);
    let page = value.unwrap();

    let ellipsis = html! {<PaginationEllipsis> {"\u{2026}"} </PaginationEllipsis>};

    let left_ellipsis = (page >= 4).then(|| ellipsis.clone()).unwrap_or_default();

    let right_ellipsis = (total - page >= 3).then(|| ellipsis).unwrap_or_default();

    let item = |idx| {
        let click = input.reform(move |_| idx);
        html! { <PaginationLink {click} current={ page == idx }> {idx} </PaginationLink> }
    };

    let first = item(1);
    let last = (total != 1).then(|| item(total)).unwrap_or_default();

    let center = (page.max(3) - 1)..(page + 2).min(total);

    html! {
        <components::Pagination {size} {class} {alignment} {rounded}>
            <components::PaginationList>
                { first }
                { left_ellipsis }
                { for center.map(item) }
                { right_ellipsis }
                { last }
            </components::PaginationList>
        </components::Pagination>
    }
}
