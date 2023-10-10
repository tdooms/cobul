use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use yew::prelude::*;

use cobul_props::{Model, general::Active};

#[derive(Clone, Debug, Properties, PartialEq)]
struct MenuItemProps {
    pub text: AttrValue,
    pub model: Model<bool>,
}

#[function_component(MenuItem)]
fn menu_item(props: &MenuItemProps) -> Html {
    let onclick = props.model.input.reform(|_| true);
    let class = classes!(Active(props.model.value));
    html! { <li {onclick}> <a {class}> {props.text.clone()} </a> </li> }
}

#[derive(Clone, Debug, Properties, PartialEq)]
struct MenuListProps {
    pub label: AttrValue,
    pub list: Vec<AttrValue>,
    pub model: Model<(AttrValue, AttrValue)>,
}

#[function_component(MenuList)]
fn menu_list(props: &MenuListProps) -> Html {
    let Model { value, input } = props.model.clone();
    let k = props.label.clone();

    let mapper = move |v: &AttrValue| {
        let k_copy = k.clone();
        let v_copy = v.clone();

        let model = Model {
            value: (&value.0, &value.1) == (&k, &v),
            input: input.reform(move |_| (k_copy.clone(), v_copy.clone())),
        };
        (v.clone(), model)
    };

    let list = props.list.iter().map(mapper).collect::<Vec<_>>();

    html! {
        <>
        <p class="menu-label"> {props.label.clone()} </p>
        <ul class="menu-list">
            {for list.into_iter().map(move |(text, model)| html! { <MenuItem {text} {model} /> })}
        </ul>
        </>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub content: Rc<BTreeMap<AttrValue, Vec<AttrValue>>>,
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
