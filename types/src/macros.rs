macro_rules! ser_field {
    ($serializer:ident, $state:expr, $field:expr, $val_opt:expr) => (
        if let Some(f) = $val_opt {
            try!($serializer.serialize_struct_elt($state, $field, f));
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

        impl <M> AsRef<$std_ty> for $wrapper_ty<M> where
        M: $mapping_ty {
            fn as_ref(&self) -> &$std_ty {
                &self.value
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
    ($std_ty:ident, $wrapper_ty:ident, $mapping_ty:ident, $format_ty:ident) => (
        impl <M, F> From<$std_ty> for $wrapper_ty<F, M> where
        F: $format_ty,
        M: $mapping_ty<Format = F> {
            fn from(value: $std_ty) -> Self {
                $wrapper_ty::new(value)
            }
        }

        impl <M, F> AsRef<$std_ty> for $wrapper_ty<F, M> where
        F: $format_ty,
        M: $mapping_ty<Format = F> {
            fn as_ref(&self) -> &$std_ty {
                &self.value
            }
        }

        impl<M, F> PartialEq<$std_ty> for $wrapper_ty<F, M> where
        F: $format_ty,
        M: $mapping_ty<Format = F> {
            fn eq(&self, other: &$std_ty) -> bool {
                PartialEq::eq(&self.value, other)
            }

            fn ne(&self, other: &$std_ty) -> bool {
                PartialEq::ne(&self.value, other)
            }
        }

        impl<F, M> PartialEq<$wrapper_ty<F, M>> for $std_ty where
        F: $format_ty,
        M: $mapping_ty<Format = F>{
            fn eq(&self, other: &$wrapper_ty<F, M>) -> bool {
                PartialEq::eq(self, &other.value)
            }

            fn ne(&self, other: &$wrapper_ty<F, M>) -> bool {
                PartialEq::ne(self, &other.value)
            }
        }

        impl <M, F> ::std::ops::Deref for $wrapper_ty<F, M> where
        F: $format_ty,
        M: $mapping_ty<Format = F> {
            type Target = $std_ty;
            fn deref(&self) -> &$std_ty {
                &self.value
            }
        }
    );
}
