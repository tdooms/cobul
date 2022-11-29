use yew::prelude::*;

use crate::props::{ColumnOffset, ColumnSize};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub size: Option<ColumnSize>,

    #[prop_or_default]
    pub offset: Option<ColumnOffset>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// The power of Flexbox in a simple interface - [reference](https://bulma.io/documentation/columns/)
///
/// Properties:
/// - `size: Option<ColumnSize>`
/// - `offset: Option<ColumnOffset>`
#[function_component(Column)]
pub fn column(props: &Props) -> Html {
    let class = classes!("column", props.class.clone(), props.size, props.offset);

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
