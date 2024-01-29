use implicit_clone::unsync::{IArray, IMap};
use yew::prelude::*;

use cobul_model::Model;
use cobul_props::general::Active;

#[derive(Clone, Debug, Properties, PartialEq)]
struct MenuItemProps {
    pub text: AttrValue,
    pub model: Model<bool>,
}

#[function_component(MenuItem)]
fn menu_item(props: &MenuItemProps) -> Html {
    let onclick = props.model.reform(|_| true);
    let class = classes!(Active(props.model.value()));

    html! { <li {onclick}> <a {class}> {props.text.clone()} </a> </li> }
}

#[derive(Clone, Debug, Properties, PartialEq)]
struct MenuListProps {
    pub label: AttrValue,
    pub list: IArray<AttrValue>,

    pub model: Model<(AttrValue, AttrValue)>,
}

#[function_component(MenuList)]
fn menu_list(props: &MenuListProps) -> Html {
    let MenuListProps { label, list, .. } = props.clone();
    let cloned = props.model.clone();

    let mapper = move |item: AttrValue| {
        let value = (label.clone(), item.clone()) == *cloned;
        let (label, item) = (label.clone(), item.clone());
        let input = cloned.reform(move |_| (label.clone(), item.clone()));
        Model::new(value, input)
    };

    html! {
        <>
        <p class="menu-label"> {props.label.clone()} </p>
        <ul class="menu-list">
            {for list.iter().map(move |text| html! { <MenuItem text={text.clone()} model={mapper(text)} /> })}
        </ul>
        </>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub content: IMap<AttrValue, IArray<AttrValue>>,
    pub model: Model<(AttrValue, AttrValue)>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

// TODO: Currently, for simplicity, there is no further nesting of the menu list items.
#[function_component(Menu)]
pub fn menu_list(props: &Props) -> Html {
    let class = classes!("menu", props.class.clone());
    let model = props.model.clone();

    html! {
        <aside style={props.style.clone()} {class}>
        { for props.content.iter().map(|(label, list)| html!{ <MenuList {label} list={list.clone()} model={model.clone()} />}) }
        </aside>
    }
}
