use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A simple responsive footer which can include anything: lists, headings, columns, icons, buttons, etc. -[reference](https://bulma.io/documentation/layout/footer/)
#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {
    let class = classes!("footer", props.class.clone());
    html! {
        <footer style={props.style.clone()} {class}>
            { for props.children.iter() }
        </footer>
    }
}
