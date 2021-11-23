mod form;
mod signup;



use wasm_bindgen::prelude::*;
use yew::*;
use crate::signup::{Signup, SignupForm};
use std::rc::Rc;
use cobul::*;


pub struct Model {
    form: Rc<Signup>,
    text: String
}

pub enum Msg {
    Change(Rc<Signup>),
    Submit(Rc<Signup>),
    Cancel(Rc<Signup>)
}

impl Component for Model {
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

        Model {
            form: Rc::new(first),
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
                <SignupForm inner={self.form.clone()} onchange={link.callback(Msg::Change)} onsubmit={link.callback(Msg::Submit)} />
                {&self.text}
            </Container>
        }
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Model>();
    Ok(())
}

