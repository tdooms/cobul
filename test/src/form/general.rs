use derive_more::Display;
use strum::EnumIter;
use yew::*;
use cobul::*;
use cobul::{Button, Select};


#[derive(Display, Clone, Copy, EnumIter, PartialEq)]
pub enum SelectEnum {
    #[display(fmt = "Select dropdown")]
    SelectDropdown,
    #[display(fmt = "With options")]
    WithOptions,
}

#[function_component(GeneralTest)]
pub fn general() -> Html {
    let model0 = use_model(|| "".to_string());
    let model1 = use_model(|| "bulma".to_string());
    let model2 = use_model(|| SelectEnum::SelectDropdown);
    let model3 = use_model(|| "".to_string());

    html! {
        <>
        <Field>
            <Label> {"Name (Linked to Email field)"} </Label>
            <Control>
                <Input placeholder="Text input" model={model0.clone()} />
            </Control>
        </Field>

        <Field>
            <Label> {"Username"} </Label>
            <Control right="fas fa-user" left="fas fa-check">
                <Input color={Color::Success} placeholder="Text input" model={model1} />
                <Help color={Color::Success}> {"This username is available"} </Help>
            </Control>
        </Field>

        <Field>
            <Label> {"Email"} </Label>
            <Control right="fas fa-envelope" left="fas fa-exclamation-triangle">
                <Input color={Color::Danger} placeholder="Text input" model={model0} />
                <Help color={Color::Danger}> {"This email is invalid"} </Help>
            </Control>
        </Field>

        <Field>
            <Label> {"Subject"} </Label>
            <Control>
                <Select<SelectEnum> model={model2} />
            </Control>
        </Field>

        <Field>
            <Label> {"Message"} </Label>
            <Control>
                <Textarea placeholder="Textarea" model={model3} />
            </Control>
        </Field>

        <Field grouped=true>
            <Control>
                <Button color={Color::Link} text="Submit" />
            </Control>
            <Control>
                <Button color={Color::Link} light=true text="Cancel" />
            </Control>
        </Field>
        </>
    }
}