use yew::prelude::*;
use yew::virtual_dom::VChild;

use crate::elements::Image;
use crate::utils::enclose;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub header: Option<Html>,

    #[prop_or_default]
    pub image: Option<VChild<Image>>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub footer: Option<Html>,

    #[prop_or_default]
    pub fullheight: bool,

    #[prop_or_default]
    pub class: Classes,
}

/// [// https://bulma.io/documentation/components/card/](// https://bulma.io/documentation/components/card/)
#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let style = "height:100%;display:flex;flex-direction:column";
    let classes = classes!("card", props.class.clone());

    html! {
        <div class={classes} style={props.fullheight.then(|| style)}>
            { enclose("card-header", props.header.clone()) }
            { enclose("card-image", props.image.clone().map(Html::from)) }
            <div class="card-content" style={props.fullheight.then(|| "height:100%")}>
                { for props.children.iter() }
            </div>
            { enclose("card-footer", props.footer.clone()) }
        </div>
    }
}
