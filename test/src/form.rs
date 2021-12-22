use yew::*;
use validator::Validate;
use cobul::props::Color;
use cobul::*;


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

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Change(Signup),
    Submit(Signup),
    Cancel(Signup)
}

#[function_component(SignupForm)]
pub fn signup(props: &Form<Signup, Msg>) -> Html {
    let Signup { mail, site, username, age } = props.values();
    let errors = props.errors();

    html! {
        <>
        <SimpleField label="mail" help_color={Color::Danger} help={errors.get("mail").cloned()}>
            <Input oninput={props.field(|v, x| x.mail = v, Msg::Change)} value={mail}/>
        </SimpleField>

        <SimpleField label="site" help_color={Color::Danger} help={errors.get("site").cloned()}>
            <Input oninput={props.field(|v, x| x.site = v, Msg::Change)} value={site}/>
        </SimpleField>

        <SimpleField label="username" help_color={Color::Danger} help={errors.get("username").cloned()}>
            <Input oninput={props.field(|v, x| x.username = v, Msg::Change)} value={username}/>
        </SimpleField>

        <SimpleField label="age" help_color={Color::Danger} help={errors.get("age").cloned()}>
            <Input oninput={props.field(|v: String, x| x.age = v.parse().unwrap_or_default(), Msg::Change)} value={age.to_string()}/>
        </SimpleField>

        <Buttons>
            <Button onclick={props.callback(Msg::Submit)} color={Color::Primary}> {"Submit"} </Button>
            <Button onclick={props.callback(Msg::Cancel)} color={Color::Danger}> {"Cancel"} </Button>
        </Buttons>

        <p> {format!("{:?}", errors)} </p>
        </>
    }
}

// --------------------------------------------- //

pub struct TestForm {
    form: Signup,
    text: String
}

impl Component for TestForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let first = Signup {
            mail: "thomas@dooms.eu".to_owned(),
            site: "https://www.youtube.com".to_owned(),
            username: "dumos".to_owned(),
            age: 19,
        };

        // let second = Signup {
        //     mail: "nomail".to_owned(),
        //     site: "nosite".to_owned(),
        //     username: "xXXx".to_owned(),
        //     age: 9000,
        // };

        TestForm {
            form: first,
            text: String::new()
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(data) => self.form = data,
            Msg::Submit(_) => self.text = "submitted".to_owned(),
            Msg::Cancel(_) => self.text = "cancelled".to_owned(),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <Container>
                <SignupForm inner={self.form.clone()} callback={link.callback(|x| x)} />
                {&self.text}
            </Container>
        }
    }
}