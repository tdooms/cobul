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
    pub style: String,
}

/// [// https://bulma.io/documentation/components/card/](// https://bulma.io/documentation/components/card/)
#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let css = "height:100%;display:flex;flex-direction:column;";
    let style = format!("{}{}", props.fullheight.then(|| css).unwrap_or_default(), props.style);
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
