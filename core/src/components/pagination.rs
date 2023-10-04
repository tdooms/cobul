use yew::*;

use cobul_raw::components;
use cobul_raw::components::{PaginationEllipsis, PaginationLink};
use cobul_props::{Align, Size, Model};

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
    pub align: Align,

    #[prop_or_default]
    pub rounded: bool,

    pub total: u64,
}

/// A responsive, usable, and flexible pagination - [reference](https://bulma.io/documentation/components/pagination/)
#[function_component(Pagination)]
pub fn pagination(props: &Props) -> Html {
    let Props {
        size,
        class,
        align,
        rounded,
        total,
        ..
    } = props.clone();

    let (input, value) = Model::combine(&props.input, &props.value, &props.model);
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
        <components::Pagination {size} {class} {align} {rounded}>
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
