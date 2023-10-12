use strum::{Display, EnumIter};
use yew::*;
use cobul::*;

use implicit_clone::ImplicitClone;
use implicit_clone::unsync::IString;
use validator::Validate;


#[derive(Copy, Clone, Debug, PartialEq, EnumIter, Display, Default)]
pub enum Gender {
    #[default]
    Man,
    Woman,
    Other
}

#[derive(Form, Debug, Clone, Default, PartialEq, Validate)]
pub struct Person {
    #[validate(length(min = 1, message = "First name is required"))]
    #[validate(length(max = 5, message = "First name must be less than 5 characters"))]
    first: IString,

    #[validate(length(max = 5, message = "Long last names are awesome!"))]
    last: IString,

    gender: Gender,
    height: f32,
}

impl ImplicitClone for Person {}

#[function_component(DeriveTest)]
pub fn derive_test() -> Html {
    let form = use_form::<Person>();

    html! {
        <>
        <Field label="First name" help={form.error("first")} color={Color::Danger}>
            <Input model={form.first()} />
        </Field>

        <Field label="Last name" help={form.error("last")} color={Color::Success}>
            <Input model={form.last()} />
        </Field>

        <Field label="Gender">
            <simple::Tabs<Gender> model={form.gender()} toggle=true />
        </Field>

        <Field label="Height">
            <extra::Slider<f32> model={form.height()} range={0.0..2.0} step=0.01 label=true fmt="{}m" label_width=4.0 />
        </Field>

        <Notification color={Color::Danger} light=true> {format!("{:?}", form.errors(false))} </Notification>
        <Notification> {format!("{:?}", form.value())} </Notification>
        </>
    }
}