use yew::prelude::*;

use crate::utils::enclose_with_tag;

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

    #[prop_or("div".to_owned())]
    pub right_tag: String,

    #[prop_or("div".to_owned())]
    pub left_tag: String,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
#[function_component(Media)]
pub fn media(props: &Props) -> Html {
    let classes = classes!("media", props.class.clone());
    html! {
        <div style={props.style.clone()} class={classes}>
            { enclose_with_tag(props.left_tag.clone(), "media-left", props.left.clone()) }
            <div class="media-content"> { for props.children.iter() } </div>
            { enclose_with_tag(props.right_tag.clone(), "media-right", props.right.clone()) }
        </div>
    }
}
