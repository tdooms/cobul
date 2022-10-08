use crate::utils::enclose_tag;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub right: Option<Html>,

    #[prop_or_default]
    pub left: Option<Html>,

    #[prop_or_else(|| "div".into())]
    pub right_tag: AttrValue,

    #[prop_or_else(|| "div".into())]
    pub left_tag: AttrValue,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// The famous media object prevalent in social media interfaces, but useful in any context - [reference](https://bulma.io/documentation/layout/media-object/)
#[function_component(Media)]
pub fn media(props: &Props) -> Html {
    let class = classes!("media", props.class.clone());
    html! {
        <div style={props.style.clone()} {class}>
            { enclose_tag(props.left_tag.to_string(), "media-left", props.left.clone()) }
            <div class="media-content"> { for props.children.iter() } </div>
            { enclose_tag(props.right_tag.to_string(), "media-right", props.right.clone()) }
        </div>
    }
}
