macro_rules! ser_field {
    ($serializer:ident, $field:expr, $val_opt:expr) => {
        if let Some(f) = $val_opt {
            $serializer.serialize_field($field, &f)?;
        }
    };
}

macro_rules! borrow_fn {
    ($std_ty:ident) => {
        fn borrow<T>(value: &T) -> &$std_ty
        where
            T: Borrow<$std_ty>,
        {
            value.borrow()
        }
    };
}

macro_rules! impl_mapping_type {
    ($std_ty:ident, $wrapper_ty:ident, $mapping_ty:ident) => {
        impl<M> crate::types::private::field::StdField<$std_ty> for $wrapper_ty<M> where
            M: $mapping_ty
        {
        }

        impl<M> From<$std_ty> for $wrapper_ty<M>
        where
            M: $mapping_ty,
        {
            fn from(value: $std_ty) -> Self {
                $wrapper_ty::new(value)
            }
        }

        impl<M> PartialEq<$std_ty> for $wrapper_ty<M>
        where
            M: $mapping_ty,
        {
            fn eq(&self, other: &$std_ty) -> bool {
                borrow_fn!($std_ty);

                PartialEq::eq(borrow(&self.value), other)
            }

            fn ne(&self, other: &$std_ty) -> bool {
                borrow_fn!($std_ty);

                PartialEq::ne(borrow(&self.value), other)
            }
        }

        impl<M> PartialEq<$wrapper_ty<M>> for $std_ty
        where
            M: $mapping_ty,
        {
            fn eq(&self, other: &$wrapper_ty<M>) -> bool {
                borrow_fn!($std_ty);

                PartialEq::eq(self, borrow(&other.value))
            }

            fn ne(&self, other: &$wrapper_ty<M>) -> bool {
                borrow_fn!($std_ty);

                PartialEq::ne(self, borrow(&other.value))
            }
        }

        impl<M> ::std::ops::Deref for $wrapper_ty<M>
        where
            M: $mapping_ty,
        {
            type Target = $std_ty;
            fn deref(&self) -> &$std_ty {
                borrow_fn!($std_ty);

                borrow(&self.value)
            }
        }

        impl<M> ::std::borrow::Borrow<$std_ty> for $wrapper_ty<M>
        where
            M: $mapping_ty,
        {
            fn borrow(&self) -> &$std_ty {
                borrow_fn!($std_ty);

                borrow(&self.value)
            }
        }
    };
}
