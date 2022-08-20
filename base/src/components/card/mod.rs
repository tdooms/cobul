pub use footer_item::CardFooterItem;
pub use header_icon::CardHeaderIcon;
pub use header_title::CardHeaderTitle;

mod footer_item;
mod header_icon;
mod header_title;

use crate::utils::enclose;
use yew::prelude::*;

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
    pub style: Option<String>,
}

/// [// https://bulma.io/documentation/components/card/](// https://bulma.io/documentation/components/card/)
#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let css = "height:100%;display:flex;flex-direction:column";
    let style = match (props.fullheight, &props.style) {
        (true, Some(style)) => format!("{};{}", css, style),
        (false, Some(style)) => style.clone(),
        (true, None) => css.to_string(),
        (false, None) => String::new(),
    };

    let classes = classes!("card", props.class.clone());

    html! {
        <div class={classes} {style}>
            { enclose("card-header", props.header.clone()) }
            { enclose("card-image", props.image.clone().map(Html::from)) }
            <div class="card-content" style={props.fullheight.then(|| "height:100%")}>
                { for props.children.iter() }
            </div>
            { enclose("card-footer", props.footer.clone()) }
        </div>
    }
}
