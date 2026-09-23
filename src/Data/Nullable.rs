// Native payload for the existing typed Nullable FFI; handles share immutable values.
use std::rc::Rc;

pub struct Nullable {
    value: Option<crate::UnknownType>,
}

impl Nullable {
    /// Reads the wrapped value, preserving a present `Nullable a` wrapper.
    pub fn value(&self) -> Option<crate::UnknownType> {
        self.value.clone()
    }
}

pub fn Data_Nullable_null() -> Rc<Nullable> {
    Rc::new(Nullable { value: None })
}

pub fn Data_Nullable_notNull(value: crate::UnknownType) -> Rc<Nullable> {
    // Collapse a nested null, but retain a present typed wrapper as the payload:
    // the generated callback for Nullable (Nullable a) expects that wrapper.
    if let crate::Value::Class(payload) = value.resolve() {
        if let Some(nested) = payload.downcast_ref::<Rc<Nullable>>() {
            if nested.value.is_none() {
                return nested.clone();
            }
        }
    }
    Rc::new(Nullable { value: Some(value) })
}

pub fn Data_Nullable_nullable() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Static(|wrapped, fallback, callback| {
        let nullable = wrapped.unwrap_class::<Rc<Nullable>>();
        match &nullable.value {
            None => fallback,
            Some(value) => callback.unwrap_func1()(value.clone()),
        }
    }))
}
