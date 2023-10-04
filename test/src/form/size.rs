use yew::*;
use cobul::*;
use cobul::icons::Solid::MagnifyingGlass;

#[function_component(FormSize)]
pub fn form_size() -> Html {
    html! {
        <>
        <Field size={Size::Small} left={MagnifyingGlass}>
            <Input placeholder="search"/>
        </Field>

        <Field size={Size::Normal} left={MagnifyingGlass}>
            <Input placeholder="search"/>
        </Field>

        <Field size={Size::Medium} left={MagnifyingGlass}>
            <Input placeholder="search"/>
        </Field>

        <Field size={Size::Large} left={MagnifyingGlass}>
            <Input placeholder="search"/>
        </Field>
        </>
    }
}