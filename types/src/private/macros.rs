macro_rules! ser_field {
    ($serializer:ident, $field:expr, $val_opt:expr) => (
        if let Some(f) = $val_opt {
            try!($serializer.serialize_field($field, &f));
        }
    )
}

macro_rules! impl_mapping_type {
    ($std_ty:ident, $wrapper_ty:ident, $mapping_ty:ident) => (
        impl <M> From<$std_ty> for $wrapper_ty<M> where
        M: $mapping_ty {
            fn from(value: $std_ty) -> Self {
                $wrapper_ty::new(value)
            }
        }

        impl<M> PartialEq<$std_ty> for $wrapper_ty<M> where
        M: $mapping_ty {
            fn eq(&self, other: &$std_ty) -> bool {
                PartialEq::eq(&self.value, other)
            }

            fn ne(&self, other: &$std_ty) -> bool {
                PartialEq::ne(&self.value, other)
            }
        }

        impl<M> PartialEq<$wrapper_ty<M>> for $std_ty where
        M: $mapping_ty {
            fn eq(&self, other: &$wrapper_ty<M>) -> bool {
                PartialEq::eq(self, &other.value)
            }

            fn ne(&self, other: &$wrapper_ty<M>) -> bool {
                PartialEq::ne(self, &other.value)
            }
        }

        impl <M> ::std::ops::Deref for $wrapper_ty<M> where
        M: $mapping_ty {
            type Target = $std_ty;
            fn deref(&self) -> &$std_ty {
                &self.value
            }
        }
    );
}
