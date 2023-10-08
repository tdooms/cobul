use yew::*;
use cobul_props::{Align, Size, general::Addons};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub align: Option<Align>,

    #[prop_or_default]
    pub addons: Addons,

    #[prop_or_default]
    pub size: Option<Size>,
}

/// A list of buttons - [reference](https://bulma.io/documentation/elements/button/#list-of-buttons)
///
/// Properties:
/// - `children: Children`
/// - `class: Classes`
/// - `style: Option<AttrValue>`
/// - `align: Option<Align>`
/// - `addons: Addons`
/// - `size: Option<Size>`
#[function_component(Buttons)]
pub fn buttons(props: &Props) -> Html {
    let class = classes!("buttons", props.class.clone(), props.align, props.addons);

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}