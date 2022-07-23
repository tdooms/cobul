use std::collections::HashMap;
use std::rc::Rc;

use validator::Validate;
use yew::{hook, use_effect_with_deps, use_state_eq, Callback, UseStateHandle};

use crate::actions::Actions;
use crate::errors::{use_errors, UseErrorHandle};

#[derive(Clone, PartialEq)]
pub struct UseFormHandle<T: Validate + Clone> {
    state: UseStateHandle<Rc<T>>,
    actions: Actions<T>,
    errors: UseErrorHandle,
}

impl<T: Validate + Clone + 'static> UseFormHandle<T> {
    pub fn submit(&self) -> Callback<()> {
        let state = self.state.clone();
        self.actions
            .onsubmit
            .clone()
            .unwrap_or_default()
            .reform(move |_| (*state).clone())
    }

    pub fn cancel(&self) -> Callback<()> {
        let state = self.state.clone();
        self.actions
            .oncancel
            .clone()
            .unwrap_or_default()
            .reform(move |_| (*state).clone())
    }

    pub fn error(&self, key: &str) -> Option<String> {
        self.errors.get(key)
    }

    pub fn errors(&self) -> HashMap<String, String> {
        self.errors.all()
    }

    pub fn can_submit(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn field<F>(&self, extract: impl Fn(&mut T) -> &mut F + 'static) -> Callback<F> {
        let Self {
            state,
            actions: Actions { onchange, .. },
            ..
        } = self.clone();

        Callback::from(move |value| {
            let mut inner = (**state).clone();
            *extract(&mut inner) = value;
            let new = Rc::new(inner);

            state.set(new.clone());
            onchange.clone().map(|cb| cb.emit(new));
        })
    }

    pub fn maybe_field<F>(
        &self,
        extract: impl Fn(&mut T) -> &mut F + 'static,
        field: &'static str,
        explain: impl Fn(String) -> String + 'static,
    ) -> Callback<Result<F, String>> {
        let Self {
            state,
            actions: Actions { onchange, .. },
            errors,
        } = self.clone();

        Callback::from(move |result| match result {
            Ok(value) => {
                let mut inner = (**state).clone();
                *extract(&mut inner) = value;
                let new = Rc::new(inner);

                onchange.clone().map(|cb| cb.emit(new.clone()));
                state.set(new);

                errors.set(field.clone(), None);
            }
            Err(err) => errors.set(field.clone(), Some(explain(err))),
        })
    }
}

#[hook]
pub fn use_form<T>(value: Rc<T>, actions: Actions<T>) -> UseFormHandle<T>
where
    T: Validate + PartialEq + Clone + 'static,
{
    let cloned = value.clone();
    let state = use_state_eq(move || cloned);

    let cloned = state.clone();
    use_effect_with_deps(
        move |value| {
            cloned.set(value.clone());
            || ()
        },
        value.clone(),
    );

    let errors = use_errors(state.as_ref());

    let handle = UseFormHandle {
        state,
        errors,
        actions,
    };

    handle
}
