use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// [// https://bulma.io/documentation/components/dropdown/](// https://bulma.io/documentation/components/dropdown/)
#[function_component(DropdownDivider)]
pub fn dropdown_divider(props: &Props) -> Html {
    let class = classes!("dropdown-divider", props.class.clone());
    html! { <hr style={props.style.clone()} {class} /> }
}
