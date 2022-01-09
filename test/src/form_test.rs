use cobul::props::Color;
use cobul::*;
use validator::Validate;
use yew::*;

#[derive(Debug, Clone, Validate, PartialEq)]
pub struct Signup {
    #[validate(email(message = "must be valid email"))]
    pub mail: String,

    #[validate(url(message = "string must be valid site"))]
    pub site: String,

    #[validate(length(min = 1))]
    pub username: String,
    #[validate(range(min = 18, max = 20, message = "age must be between 18 and 20"))]
    pub age: u32,
}

#[function_component(SignupForm)]
pub fn signup(props: &Form<Signup>) -> Html {
    let Signup {
        mail,
        site,
        username,
        age
    } = props.inner();
    let errors = props.errors();

    html! {
        <>
        <SimpleField label="mail" help_color={Color::Danger} help={errors.get("mail").cloned()}>
            <Input oninput={props.field(|x| &mut x.mail)} value={mail}/>
        </SimpleField>

        <SimpleField label="site" help_color={Color::Danger} help={errors.get("site").cloned()}>
            <Input oninput={props.field(|x| &mut x.site)} value={site}/>
        </SimpleField>

        <SimpleField label="username" help_color={Color::Danger} help={errors.get("username").cloned()}>
            <Input oninput={props.field(|x| &mut x.username)} value={username}/>
        </SimpleField>

        <SimpleField label="age" help_color={Color::Danger} help={errors.get("age").cloned()}>
            <Input oninput={props.change(|x, f: String| x.age = f.parse::<u32>().unwrap_or_default())} value={age.to_string()}/>
        </SimpleField>

        <Buttons>
            <Button onclick={props.submit()} color={Color::Primary}> {"Submit"} </Button>
            <Button onclick={props.cancel()} color={Color::Danger}> {"Cancel"} </Button>
            <Button onclick={props.reset()} color={Color::Warning}> {"Reset"} </Button>
        </Buttons>

        <p> {format!("{:?}", errors)} </p>
        </>
    }
}

// --------------------------------------------- //

#[function_component(FormTester)]
pub fn form_tester() -> Html {
    let state = use_state(|| Signup {
        mail: "thomas@dooms.eu".to_owned(),
        site: "https://www.youtube.com".to_owned(),
        username: "dumos".to_owned(),
        age: 19,
    });

    let onchange = {
        let state = state.clone();
        Callback::from(move |signup| state.set(signup))
    };

    html! {
        <Container>
            <SignupForm inner={(*state).clone()} onchange={onchange}/>
        </Container>
    }
}
