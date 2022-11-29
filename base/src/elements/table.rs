use yew::prelude::*;

use crate::props::{Bordered, Fullwidth, Hoverable, Narrow, Striped};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub bordered: Bordered,

    #[prop_or_default]
    pub striped: Striped,

    #[prop_or_default]
    pub narrow: Narrow,

    #[prop_or_default]
    pub hoverable: Hoverable,

    #[prop_or_default]
    pub fullwidth: Fullwidth,

    #[prop_or_default]
    pub scrollable: bool,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// The inevitable HTML table, with special case cells - [reference](https://bulma.io/documentation/elements/table/)
///
/// Properties:
/// - `bordered: Bordered`
/// - `striped: Striped`
/// - `narrow: Narrow`
/// - `hoverable: Hoverable`
/// - `fullwidth: Fullwidth`
/// - `scrollable: bool`
#[function_component(Table)]
pub fn table(props: &Props) -> Html {
    let class = classes!(
        "table",
        props.class.clone(),
        props.bordered,
        props.striped,
        props.narrow,
        props.hoverable,
        props.fullwidth,
    );

    let table = html! {
        <table style={props.style.clone()} {class}>
            { for props.children.iter() }
        </table>
    };

    match props.scrollable {
        true => html! {<div class="table-container"> {table} </div> },
        false => table,
    }
}
