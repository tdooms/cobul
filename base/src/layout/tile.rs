use yew::prelude::*;

use crate::props::{TileCtx, TileSize, Vertical};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_else(|| "div".into())]
    pub tag: AttrValue,

    #[prop_or_default]
    pub ctx: Option<TileCtx>,

    #[prop_or_default]
    pub vertical: Vertical,

    #[prop_or_default]
    pub size: Option<TileSize>,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// [https://bulma.io/documentation/layout/tiles/](https://bulma.io/documentation/layout/tiles/)
#[function_component(Tile)]
pub fn tile(props: &Props) -> Html {
    let class = classes!(
        "tile",
        props.class.clone(),
        props.ctx,
        props.size,
        props.vertical
    );

    html! {
        <@{ props.tag.to_string() } style={props.style.clone()} {class}>
            { for props.children.iter() }
        </@>
    }
}
