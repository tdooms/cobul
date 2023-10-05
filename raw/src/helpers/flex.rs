use yew::*;
use cobul_props::{FlexAlignContent, FlexAlignItems, FlexAlignSelf, FlexDirection, FlexJustify, FlexSize, FlexWrap};


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub direction: Option<FlexDirection>,

    #[prop_or_default]
    pub wrap: Option<FlexWrap>,

    #[prop_or_default]
    pub justify: Option<FlexJustify>,

    #[prop_or_default]
    pub align_content: Option<FlexAlignContent>,

    #[prop_or_default]
    pub align_items: Option<FlexAlignItems>,

    #[prop_or_default]
    pub align_self: Option<FlexAlignSelf>,

    #[prop_or_default]
    pub size: Option<FlexSize>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}


#[function_component(Flex)]
pub fn flex(props: &Props) -> Html {
    let class = classes!("is-flex", props.class.clone(), props.direction, props.wrap, props.justify, props.align_content, props.align_items, props.align_self, props.size);

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}