use yew::prelude::*;

use crate::props::{Addons, AddonsAlignment, Grouped, GroupedMultiline};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub grouped: Grouped,

    #[prop_or_default]
    pub multiline: GroupedMultiline,

    #[prop_or_default]
    pub addons: Addons,

    #[prop_or_default]
    pub alignment: AddonsAlignment,

    #[prop_or_default]
    pub style: String,
}

/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
#[function_component(Field)]
pub fn field(props: &Props) -> Html {
    let classes = classes!(
        "field",
        props.class.clone(),
        props.multiline,
        props.addons,
        props.grouped,
        props.alignment
    );

    html! {
        <div style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </div>
    }
}
