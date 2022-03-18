use validator::Validate;
use yew::*;

use cobul::*;
use cobul::props::Color;

#[derive(Debug, Clone, Validate, PartialEq)]
pub struct Signup {
    #[validate(email(message = "must be a valid email"))]
    pub mail: String,

    #[validate(url(message = "string must be a valid site"))]
    pub site: String,

    #[validate(length(min = 1))]
    pub username: String,
    #[validate(range(min = 18, max = 20, message = "age must be between 18 and 20"))]
    pub age: u32,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    signup: Signup,
}

#[function_component(SignupForm)]
pub fn signup(props: &Props) -> Html {
    let (form, signup) = use_form(&props.signup, FormActions::default());
    let Signup { mail, site, username, age } = signup;

    html! {
        <>
        <SimpleField label="mail" help_color={Color::Danger} help={form.error("mail")}>
            <Input oninput={form.field(|x| &mut x.mail)} value={mail}/>
        </SimpleField>

        <SimpleField label="site" help_color={Color::Danger} help={form.error("site")}>
            <Input oninput={form.field(|x| &mut x.site)} value={site}/>
        </SimpleField>

        <SimpleField label="username" help_color={Color::Danger} help={form.error("username")}>
            <Input oninput={form.field(|x| &mut x.username)} value={username}/>
        </SimpleField>

        <SimpleField label="age" help_color={Color::Danger} help={form.error("age")}>
            <TypedInput<u32> oninput={form.field(|x| &mut x.age)}} value={age}/>
        </SimpleField>

        <Buttons>
            <Button onclick={form.submit()} color={Color::Primary}> {"Submit"} </Button>
            <Button onclick={form.cancel()} color={Color::Danger}> {"Cancel"} </Button>
        </Buttons>
        </>
    }
}

// --------------------------------------------- //

#[function_component(FormTester)]
pub fn form_tester() -> Html {
    let state = use_state(|| false);
    let checked = (*state).clone();

    let signup1 = Signup {
        mail: "thomas@dooms.eu".to_owned(),
        site: "https://www.youtube.com".to_owned(),
        username: "dumos".to_owned(),
        age: 19,
    };

    let signup2 = Signup {
        mail: "silky@way.com".to_owned(),
        site: "https://www.pixelguesser.com".to_owned(),
        username: "wuvve".to_owned(),
        age: 20,
    };

    let signup = if checked { signup1 } else { signup2 };
    let onchange = Callback::from(move |b| state.set(b));

    html! {
        <Container>
            <SignupForm {signup}/>
            <Checkbox name={"checkbox"} {checked} {onchange} />
        </Container>
    }
}
