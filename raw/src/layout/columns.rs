use yew::prelude::*;

use cobul_props::{Breakpoint};
use cobul_props::general::{Centered, Gapless, Multiline, VCentered};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub vcentered: VCentered,

    #[prop_or_default]
    pub multiline: Multiline,

    #[prop_or_default]
    pub centered: Centered,

    #[prop_or_default]
    pub gapless: Gapless,

    #[prop_or_default]
    pub breakpoint: Option<Breakpoint>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// The power of Flexbox in a simple interface - [reference](https://bulma.io/documentation/columns/)
///     
/// Properties:
/// - `vcentered: VCentered`
/// - `multiline: Multiline`
/// - `centered: Centered`
/// - `gapless: Gapless`
/// - `breakpoint: Option<Breakpoint>`
#[function_component(Columns)]
pub fn columns(props: &Props) -> Html {
    let class = classes!(
        "columns",
        props.class.clone(),
        props.vcentered,
        props.multiline,
        props.centered,
        props.gapless
    );

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
