// use crate::form::Form;
// use yew::*;
// use validator::Validate;
// use cobul::props::Color;
// use cobul::*;
//
// #[derive(Debug, Clone, Validate, PartialEq)]
// pub struct Signup {
//     #[validate(email(message = "must be valid email"))]
//     pub mail: String,
//
//     #[validate(url(message = "string must be valid site"))]
//     pub site: String,
//
//     #[validate(length(min = 1))]
//     pub username: String,
//
//     #[validate(range(min = 18, max = 20, message = "age must be between 18 and 20"))]
//     pub age: u32,
// }
//
// #[function_component(SignupForm)]
// pub fn signup(props: &Form<Signup>) -> Html {
//     let errors = props.errors();
//
//     let Signup {
//         mail, site, username, age
//     } = (*props.inner).clone();
//
//     html! {
//         <>
//         <SimpleField label="mail" help_color={Color::Danger} help={errors.get("mail").cloned()}>
//             <Input oninput={props.callback(|mail, x| x.mail = mail)} value={mail}/>
//         </SimpleField>
//
//         <SimpleField label="site" help_color={Color::Danger} help={errors.get("site").cloned()}>
//             <Input oninput={props.callback(|site, x| x.site = site)} value={site}/>
//         </SimpleField>
//
//         <SimpleField label="username" help_color={Color::Danger} help={errors.get("username").cloned()}>
//             <Input oninput={props.callback(|username, x| x.username = username)} value={username}/>
//         </SimpleField>
//
//         <SimpleField label="age" help_color={Color::Danger} help={errors.get("age").cloned()}>
//             <Input oninput={props.callback(|age: String, x| x.age = age.parse().unwrap_or_default())} value={age.to_string()}/>
//         </SimpleField>
//
//         <Buttons>
//             <Button color={Color::Primary}> {"Submit"} </Button>
//             <Button color={Color::Danger}> {"Cancel"} </Button>
//         </Buttons>
//
//         <p> {format!("{:?}", errors)} </p>
//         </>
//     }
// }