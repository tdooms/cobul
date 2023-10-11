use strum::{Display, EnumIter};
use yew::*;
use cobul::*;

use implicit_clone::ImplicitClone;


#[derive(Copy, Clone, Debug, PartialEq, EnumIter, Display, Default)]
pub enum Gender {
    #[default]
    Man,
    Woman,
    Other
}

#[derive(Form, Debug, Clone, Default)]
pub struct Person {
    first: AttrValue,
    last: AttrValue,

    gender: Gender,
    height: f32,
}

impl ImplicitClone for Person {}

#[function_component(DeriveTest)]
pub fn derive_test() -> Html {
    let form = use_form::<Person>();

    html! {
        <>
        <Field label="First name"> <Input model={form.first()} /> </Field>
        <Field label="Last name"> <Input model={form.last()} /> </Field>
        <Field label="Gender"> <simple::Tabs<Gender> model={form.gender()} toggle=true /> </Field>
        <extra::Slider<f32> model={form.height()} range={0.0..2.0} step=0.01 label=true />

        <Notification> {format!("{:?}", *form)} </Notification>
        </>
    }
}