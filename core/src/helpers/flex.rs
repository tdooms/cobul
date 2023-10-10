use yew::*;

use cobul_props::{AlignContent, AlignItems, AlignSelf, Direction, Grow, Justify, Shrink, Wrap};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub direction: Option<Direction>,

    #[prop_or_default]
    pub wrap: Option<Wrap>,

    #[prop_or_default]
    pub justify: Option<Justify>,

    #[prop_or_default]
    pub align_content: Option<AlignContent>,

    #[prop_or_default]
    pub align_items: Option<AlignItems>,

    #[prop_or_default]
    pub align_self: Option<AlignSelf>,

    #[prop_or_default]
    pub grow: Grow,

    #[prop_or_default]
    pub shrink: Shrink,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Flex)]
pub fn flex(props: &Props) -> Html {
    let class = classes!(
        "is-flex",
        props.class.clone(),
        props.direction,
        props.wrap,
        props.justify,
        props.align_content,
        props.align_items,
        props.align_self,
        props.grow,
        props.shrink
    );

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
