use yew::prelude::*;

use crate::props::{TileCtx, TileSize, Vertical};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_else(|| "div".into())]
    pub tag: String,

    #[prop_or_default]
    pub ctx: Option<TileCtx>,

    #[prop_or_default]
    pub vertical: Vertical,

    #[prop_or_default]
    pub size: Option<TileSize>,
}

/// [https://bulma.io/documentation/layout/tiles/](https://bulma.io/documentation/layout/tiles/)
#[function_component(Tile)]
pub fn tile(props: &Props) -> Html {
    let classes = classes!(
        "tile",
        props.class.clone(),
        props.ctx,
        props.size,
        props.vertical
    );

    html! {
        <@{ props.tag.clone() } class={classes}>
            { for props.children.iter() }
        </@>
    }
}
