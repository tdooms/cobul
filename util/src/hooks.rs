use yew::*;

#[hook]
pub fn use_value_state<T>(value: &T) -> UseStateHandle<T> where T: PartialEq + Clone + 'static {
    let handle = use_state(|| value.clone());
    let result = handle.clone();

    let cloned = value.clone();

    use_effect_with_deps(move |_| {
        handle.set(cloned.clone());
        || ()
    }, value.clone());

    result
}
