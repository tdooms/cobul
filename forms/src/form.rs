use crate::errors::validate_fields;
use std::collections::HashMap;
use validator::Validate;
use yew::{Callback, Properties};


#[derive(Properties, PartialEq, Clone)]
pub struct Form<Ty: 'static + Validate + PartialEq + Clone> {
    pub inner: Ty,

    #[prop_or_default]
    pub onchange: Callback<Ty>,

    #[prop_or_default]
    pub onsubmit: Callback<Ty>,

    #[prop_or_default]
    pub oncancel: Callback<Ty>,
}

impl<Ty: 'static + Validate + PartialEq + Clone> Form<Ty> {
    pub fn errors(&self) -> HashMap<String, String> {
        validate_fields(&self.inner)
    }

    pub fn inner(&self) -> Ty {
        self.inner.clone()
    }

    pub fn field<Field>(
        &self,
        extract: impl Fn(&mut Ty) -> &mut Field + 'static,
    ) -> Callback<Field> {
        let cloned = self.inner.clone();
        self.onchange.reform(move |field| {
            let mut inner = cloned.clone();
            *extract(&mut inner) = field;
            inner
        })
    }

    pub fn change<Field>(&self, func: impl Fn(&mut Ty, Field) + 'static) -> Callback<Field> {
        let cloned = self.inner.clone();
        self.onchange.reform(move |field| {
            let mut inner = cloned.clone();
            func(&mut inner, field);
            inner
        })
    }

    pub fn submit(&self) -> Callback<()> {
        let cloned = self.inner.clone();
        self.onsubmit.reform(move |_| cloned.clone())
    }

    pub fn cancel(&self) -> Callback<()> {
        let cloned = self.inner.clone();
        self.oncancel.reform(move |_| cloned.clone())
    }
}
