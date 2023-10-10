use yew::prelude::*;

use cobul_props::{TileCtx, TileSize};
use cobul_props::general::Vertical;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
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

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// A single tile element to build 2-dimensional Metro-like, Pinterest-like, or whatever-you-like grids - [reference](https://bulma.io/documentation/layout/tiles/)
///
/// Properties:
/// - `tag: AttrValue` &npbs; - The HTML tag to use for the tile, default: `div`
/// - `ctx: Option<TileCtx>`
/// - `vertical: Vertical`
/// - `size: Option<TileSize>`
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
