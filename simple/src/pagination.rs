use base::components;
use base::components::{PaginationEllipsis, PaginationLink};
use base::props::{Alignment, Size};
use yew::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub rounded: bool,

    pub page: u64,

    pub total: u64,

    pub onchange: Callback<u64>,
}

#[function_component(Pagination)]
pub fn pagination(props: &Props) -> Html {
    let Props {
        size,
        class,
        alignment,
        rounded,
        page,
        total,
        onchange,
    } = props.clone();

    let ellipsis = html! {<PaginationEllipsis> {"\u{2026}"} </PaginationEllipsis>};

    let left_ellipsis = (page >= 3).then(|| ellipsis.clone()).unwrap_or_default();

    let right_ellipsis = (total - page >= 4).then(|| ellipsis).unwrap_or_default();

    let item = |idx| {
        let onclick = onchange.reform(move |_| idx);
        html! { <PaginationLink {onclick} current={ page == idx }> {idx + 1} </PaginationLink> }
    };

    let first = item(0);
    let last = (total != 1).then(|| item(total - 1)).unwrap_or_default();

    let center = (page.max(2) - 1)..(page + 2).min(total - 1);

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
