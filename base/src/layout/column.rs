use yew::prelude::*;

use crate::props::{ColumnOffset, ColumnSize};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub size: Option<ColumnSize>,

    #[prop_or_default]
    pub offset: Option<ColumnOffset>,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
#[function_component(Column)]
pub fn column(props: &Props) -> Html {
    let class = classes!("column", props.class.clone(), props.size, props.offset);

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
