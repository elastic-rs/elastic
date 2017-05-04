#[macro_export]
macro_rules! impl_string_type {
    ($wrapper_ty:ident, $mapping_ty:ident, $field_type:ident) => (
        impl <M> $field_type<M> for $wrapper_ty<M> where
        M: $mapping_ty { }

        impl_mapping_type!(String, $wrapper_ty, $mapping_ty);

        impl <M> AsRef<str> for $wrapper_ty<M> where
        M: $mapping_ty {
            fn as_ref(&self) -> &str {
                &self.value
            }
        }

        impl<'a, M> PartialEq<&'a str> for $wrapper_ty<M> where
        M: $mapping_ty {
            fn eq(&self, other: & &'a str) -> bool {
                PartialEq::eq(&self.value, *other)
            }

            fn ne(&self, other: & &'a str) -> bool {
                PartialEq::ne(&self.value, *other)
            }
        }

        impl<'a, M> PartialEq<$wrapper_ty<M>> for &'a str where
        M: $mapping_ty {
            fn eq(&self, other: &$wrapper_ty<M>) -> bool {
                PartialEq::eq(*self, &other.value)
            }

            fn ne(&self, other: &$wrapper_ty<M>) -> bool {
                PartialEq::ne(*self, &other.value)
            }
        }

        impl <M> Serialize for $wrapper_ty<M> where
        M: $mapping_ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: Serializer {
                serializer.serialize_str(&self.value)
            }
        }

        impl <'de, M> Deserialize<'de> for $wrapper_ty<M> where
        M: $mapping_ty {
            fn deserialize<D>(deserializer: D) -> Result<$wrapper_ty<M>, D::Error> where
            D: Deserializer<'de> {
                #[derive(Default)]
                struct StringVisitor<M> where
                M: $mapping_ty {
                    _m: PhantomData<M>
                }

                impl <'de, M> Visitor<'de> for StringVisitor<M> where
                M: $mapping_ty {
                    type Value = $wrapper_ty<M>;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result 
                    {
                        write!(formatter, "a json string")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<$wrapper_ty<M>, E> where
                    E: Error {
                        Ok($wrapper_ty::<M>::new(v))
                    }
                }

                deserializer.deserialize_any(StringVisitor::<M>::default())
            }
        }
    );
}