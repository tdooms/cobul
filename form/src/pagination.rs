use yew::*;

use cobul_props::{Align, Model, Size};
use cobul_core as core;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub model: Option<Model<u32>>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub align: Align,

    #[prop_or_default]
    pub rounded: bool,

    pub total: u32,
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

    let (value, input) = Model::split(&props.model);
    let page = value.unwrap_or_default();

    let ellipsis = html! {<core::PaginationEllipsis> {"\u{2026}"} </core::PaginationEllipsis>};

    let left_ellipsis = (page >= 4).then(|| ellipsis.clone()).unwrap_or_default();

    let right_ellipsis = (total - page >= 3).then(|| ellipsis).unwrap_or_default();

    let item = |idx| {
        let click = input.reform(move |_| idx);
        html! { <core::PaginationLink {click} current={ page == idx }> {idx} </core::PaginationLink> }
    };

    let first = item(1);
    let last = (total != 1).then(|| item(total)).unwrap_or_default();

    let center = (page.max(3) - 1)..(page + 2).min(total);

    html! {
        <core::Pagination {size} {class} {align} {rounded}>
            <core::PaginationList>
                { first }
                { left_ellipsis }
                { for center.map(item) }
                { right_ellipsis }
                { last }
            </core::PaginationList>
        </core::Pagination>
    }
}
