use yew::prelude::*;

use crate::props::{Addons, Alignment, Grouped, GroupedMultiline};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub focus: Callback<bool>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub grouped: Grouped,

    #[prop_or_default]
    pub multiline: GroupedMultiline,

    #[prop_or_default]
    pub addons: Addons,

    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
#[function_component(Field)]
pub fn field(props: &Props) -> Html {
    let alignment = match props.alignment {
        Some(Alignment::Centered) => "has-addons-centered",
        Some(Alignment::Right) => "has-addons-right",
        Some(Alignment::Left) => "has-addons-left",
        None => "",
    };

    let class = classes!(
        "field",
        props.class.clone(),
        props.multiline,
        props.addons,
        props.grouped,
        alignment
    );

    let onfocus = props.focus.reform(|_| true);
    let onblur = props.focus.reform(|_| false);

    html! {
        <div style={props.style.clone()} {class} {onfocus} {onblur}>
            { for props.children.iter() }
        </div>
    }
}
