use implicit_clone::ImplicitClone;
use implicit_clone::unsync::{IArray, IString};
use validator::Validate;
use yew::*;
use cobul::*;

#[derive(Form, Debug, Clone, Default, PartialEq, Validate)]
pub struct Party {
    name: IString,
    location: IString,
    members: IArray<IString>,
}

impl ImplicitClone for Party {}

#[function_component(ListsTest)]
pub fn lists() -> Html {
    let form = use_form::<Party>();
    let click = form.members().push().reform(|_| IString::default());

    html! {
        <>
        <Field label="Name"> <Input model={form.name()} placeholder="Sir Oxlong the Third" /> </Field>
        <Field label="Location"> <Input model={form.location()} placeholder="London" /> </Field>

        {for form.members().decompose().iter().map(|model| html!{
            <Box> <Input placeholder="Name" {model} /> </Box>
        })}
        <Button text="Add" {click} />
        <Notification> {format!("{:?}", form.value())} </Notification>
        </>
    }
}
