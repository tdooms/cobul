use yew::prelude::*;

use cobul_props::general::{Addons, Grouped, GroupedMultiline};
use cobul_props::Align;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub focus: Callback<bool>,

    #[prop_or_default]
    pub grouped: Grouped,

    #[prop_or_default]
    pub multiline: GroupedMultiline,

    #[prop_or_default]
    pub addons: Addons,

    #[prop_or_default]
    pub align: Option<Align>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// All generic form controls, designed for consistency - [reference](https://bulma.io/documentation/form/general/)
///
/// Properties:
/// - `focus: Callback<bool>` callback true for onfocus, false for onblur
/// - `grouped: Grouped`
/// - `multiline: GroupedMultiline`
/// - `addons: Addons`
/// - `align: Option<Align>`
#[function_component(Field)]
pub fn field(props: &Props) -> Html {
    let align = match props.align {
        Some(Align::Centered) => "has-addons-centered",
        Some(Align::Right) => "has-addons-right",
        Some(Align::Left) => "has-addons-left",
        None => "",
    };

    let class = classes!(
        "field",
        props.class.clone(),
        props.multiline,
        props.addons,
        props.grouped,
        align
    );

    let onfocus = props.focus.reform(|_| true);
    let onblur = props.focus.reform(|_| false);

    html! {
        <div style={props.style.clone()} {class} {onfocus} {onblur}>
            { for props.children.iter() }
        </div>
    }
}
