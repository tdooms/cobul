use yew::prelude::*;

use crate::props::{Bordered, Fullwidth, Hoverable, Narrow, Striped};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,

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
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/elements/table/](https://bulma.io/documentation/elements/table/)
#[function_component(Table)]
pub fn table(props: &Props) -> Html {
    let classes = classes!(
        "table",
        props.class.clone(),
        props.bordered,
        props.striped,
        props.narrow,
        props.hoverable,
        props.fullwidth,
    );

    let table = html! {
        <table style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </table>
    };

    match props.scrollable {
        true => html! {<div class="table-container"> {table} </div> },
        false => table,
    }
}
