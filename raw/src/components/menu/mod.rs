use yew::prelude::*;

pub use label::MenuLabel;
pub use list::MenuList;

mod label;
mod list;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A simple menu, for any type of vertical navigation - [reference](https://bulma.io/documentation/components/menu/)
#[function_component(Menu)]
pub fn menu(props: &Props) -> Html {
    let class = classes!("menu", props.class.clone());
    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
