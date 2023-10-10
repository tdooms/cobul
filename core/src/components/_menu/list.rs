// use yew::prelude::*;
// use cobul_props::Model;
//
// #[derive(Clone, Debug, Properties, PartialEq)]
// pub struct Props {
//     pub text: AttrValue,
//
//     #[prop_or_default]
//     pub model: Option<Model<bool>>,
//
//     #[prop_or_default]
//     pub class: Classes,
//
//     #[prop_or_default]
//     pub style: Option<AttrValue>,
// }
//
//
// // TODO: Currently, for simplicity, there is no further nesting of the menu list items.
// #[function_component(MenuList)]
// pub fn menu_list(props: &Props) -> Html {
//     let (value, input) = Model::split(&props.model);
//
//     let class = classes!("menu-list", props.class.clone());
//     html! { <p style={props.style.clone()} {class}> {props.text} </p> }
// }
