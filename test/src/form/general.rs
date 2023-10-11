use derive_more::Display;
use strum::EnumIter;
use yew::*;

use cobul::*;
use cobul::{Button, Select};
use cobul::icons::Solid;

#[derive(Display, Clone, Copy, EnumIter, PartialEq)]
pub enum SelectEnum {
    #[display(fmt = "Select dropdown")]
    SelectDropdown,
    #[display(fmt = "With options")]
    WithOptions,
}

#[function_component(GeneralTest)]
pub fn general() -> Html {
    let model0 = use_model(AttrValue::default);
    let model1 = use_model(|| "bulma".into());
    let model2 = use_model(|| SelectEnum::SelectDropdown);
    let model3 = use_model(|| "".into());

    html! {
        <>
        <Field label="Name (Linked to Email field)">
            <Input placeholder="Text input" model={model0.clone()} />
        </Field>

        <Field label="Username" help="Username is valid" right={Solid::User} left={Solid::Check} color={Color::Success}>
            <Input  placeholder="Text input" model={model1} />
        </Field>

        <Field label="Email" help="This email is invalid" right={Solid::Envelope} left={Solid::TriangleExclamation} color={Color::Danger}>
            <Input placeholder="Text input" model={model0} />
        </Field>

        <Field label="Subject">
            <Select<SelectEnum> model={model2} />
        </Field>

        <Field label="Message">
            <Textarea placeholder="Textarea" model={model3} style="resize:none" />
        </Field>

        <Field grouped=true>
            <Button color={Color::Link} text="Submit" />
            <Button color={Color::Link} light=true text="Cancel" />
        </Field>
        </>
    }
}