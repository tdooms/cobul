use yew::*;

use cobul::{Size, use_model, Field, Input};
use cobul::icons::Solid::MagnifyingGlass;

#[function_component(FieldSizeTest)]
pub fn field_size() -> Html {
    let model = use_model(AttrValue::default);
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
