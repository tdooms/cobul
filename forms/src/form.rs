use crate::util::validate_fields;
use std::collections::HashMap;
use validator::{Validate, ValidationError};
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

    #[prop_or_default]
    pub onreset: Callback<Ty>,
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

    pub fn submit(&self) -> Callback<()> {
        let cloned = self.inner.clone();
        self.onsubmit.reform(move |_| cloned.clone())
    }

    pub fn cancel(&self) -> Callback<()> {
        let cloned = self.inner.clone();
        self.oncancel.reform(move |_| cloned.clone())
    }

    pub fn reset(&self) -> Callback<()> {
        let cloned = self.inner.clone();
        self.onreset.reform(move |_| cloned.clone())
    }
}
