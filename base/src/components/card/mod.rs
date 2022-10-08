use yew::prelude::*;

pub use icon::CardHeaderIcon;
pub use item::CardFooterItem;
pub use title::CardHeaderTitle;

use crate::utils::enclose;

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
    pub children: Children,

    #[prop_or_default]
    pub footer: Option<Html>,

    #[prop_or_default]
    pub fullheight: bool,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// An all-around flexible and composable component - [reference](https://bulma.io/documentation/components/card/)
#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let css = "height:100%;display:flex;flex-direction:column";

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
