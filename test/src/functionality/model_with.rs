use yew::*;
use cobul::*;

#[derive(Properties, Clone, PartialEq)]
struct Props {
    value: u32
}

#[function_component(Inner)]
fn inner(props: &Props) -> Html {
    let model = use_model_with(props.value, |x| x * x);
    let click = model.modify(|x, _| x + 1);

    html! {
        <>
        <Button color={Color::Light} fullwidth=true text="Increment inner" {click} />
        <Block />
        <Notification color={Color::Info}  light=true> {"Inner value: "} <b>{model.value}</b> </Notification>
        </>
    }
}


#[function_component(ModelWithTest)]
pub fn model_with() -> Html {
    let model = use_model(|| 0);
    let click = model.modify(|x, _| x + 1);

    html! {
        <>
        <Button color={Color::Light} fullwidth=true text="Increment outer" {click} />
        <Block />
        <Notification color={Color::Info} light=true> {"Outer value: "} <b>{model.value}</b> </Notification>
        <hr />
        <Inner value={model.value} />
        </>
    }
}
