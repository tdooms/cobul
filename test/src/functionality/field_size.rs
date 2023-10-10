use cobul::icons::Solid::MagnifyingGlass;
use cobul::{Size, use_model};
use cobul::form::{Field, Input};
use yew::*;

#[function_component(FieldSizeTest)]
pub fn field_size() -> Html {
    let model = use_model(|| String::new());
    html! {
        <>
        <Field size={Size::Small} left={MagnifyingGlass}>
            <Input placeholder="search" model={model.clone()} />
        </Field>

        <Field size={Size::Normal} left={MagnifyingGlass}>
            <Input placeholder="search" model={model.clone()} />
        </Field>

        <Field size={Size::Medium} left={MagnifyingGlass}>
            <Input placeholder="search" model={model.clone()} />
        </Field>

        <Field size={Size::Large} left={MagnifyingGlass}>
            <Input placeholder="search" model={model.clone()} />
        </Field>
        </>
    }
}
