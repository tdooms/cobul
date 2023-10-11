use implicit_clone::ImplicitClone;
use yew::*;
use cobul::*;


#[derive(Form, Debug, Clone, Default, PartialEq)]
struct Full {
    left: AttrValue,
    right: AttrValue,
}

impl ImplicitClone for Full {}

#[derive(Properties, Clone, PartialEq)]
struct Props {
    form: FullForm
}

#[function_component(Left)]
fn left(props: &Props) -> Html {
    let model = props.form.left();
    html! { <Field> <Input {model} placeholder="Left" /> </Field> }
}

#[function_component(Right)]
fn right(props: &Props) -> Html {
    let model = props.form.right();
    html! { <Field> <Input {model} placeholder="Right" /> </Field> }
}

#[function_component(PartialTest)]
pub fn partial() -> Html {
    let form = use_form::<Full>();

    html! {
        <>
        <Right form={form.clone()} />
        <Left form={form.clone()} />
        <Notification> {format!("{:?}", *form)} </Notification>
        </>
    }
}