// use std::rc::Rc;
// use yew::{Callback, Properties};
// use std::collections::HashMap;
// use validator::{Validate, ValidationError};
//
// #[derive(Properties, PartialEq, Clone)]
// pub struct Form<T: 'static + Validate + PartialEq + Clone> {
//     pub inner: Rc<T>,
//
//     pub onchange: Option<Callback<Rc<T>>>,
//     pub onsubmit: Option<Callback<Rc<T>>>,
//     pub oncancel: Option<Callback<Rc<T>>>
// }
//
// impl<T: 'static + Validate + PartialEq + Clone> Form<T> {
//     // FIXME: this is far from efficient but I guess it works
//     // A few allocations every mouseclick is not terrible.
//     pub fn errors(&self) -> HashMap<String, String> {
//         let validated = match self.inner.validate() {
//             Ok(_) => Default::default(),
//             Err(validated) => validated
//         };
//         let errors = validated.field_errors();
//
//         let map = |(field, errors): (&str, &Vec<ValidationError>)| {
//             match errors.first().map(|err| err.message.clone().unwrap_or(err.code.clone()).into_owned()) {
//                 Some(error) => Some((field.to_owned(), error)),
//                 None => None
//             }
//         };
//
//         errors.into_iter().filter_map(map).collect()
//     }
//
//     pub fn callback<P>(&self, func: impl Fn(P, &mut T) + 'static) -> Callback<P> {
//         let cloned = self.inner.clone();
//         self.onchange.reform(move |p| {
//             let mut inner = cloned.clone();
//             func(p, Rc::make_mut(&mut inner));
//             inner
//         })
//     }
// }