use yew::prelude::*;

pub use icon::CardHeaderIcon;
pub use item::CardFooterItem;
pub use title::CardHeaderTitle;

use crate::util::enclose;

mod icon;
mod item;
mod title;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub header: Option<Html>,

    #[prop_or_default]
    pub image: Option<Html>,

    #[prop_or_default]
    pub footer: Option<Html>,

    #[prop_or_default]
    pub fullheight: bool,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,
}

/// An all-around flexible and composable component - [reference](https://bulma.io/documentation/components/card/)
///
/// Properties:
/// - `header: Option<Html>` &npbs; The header of the card
/// - `image: Option<Html>` &npbs; The image of the card
/// - `footer: Option<Html> &npbs;: The footer of the card
/// - `fullheight: bool` &npbs; CUSTOM Whether the card should take up the full height of its parent
#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let css = "height:100%;display:helpers;helpers-direction:column";

    let style = match (props.fullheight, &props.style) {
        (true, Some(style)) => format!("{};{}", css, style),
        (false, Some(style)) => style.to_string(),
        (true, None) => css.to_string(),
        (false, None) => String::new(),
    };

    let class = classes!("card", props.class.clone());

    html! {
        <div {class} {style}>
            { enclose("card-header", props.header.clone()) }
            { enclose("card-image", props.image.clone().map(Html::from)) }
            <div class="card-content" style={props.fullheight.then(|| "height:100%")}>
                { for props.children.iter() }
            </div>
            { enclose("card-footer", props.footer.clone()) }
        </div>
    }
}
