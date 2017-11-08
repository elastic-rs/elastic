macro_rules! impl_string_type {
    ($wrapper_ty:ident, $mapping_ty:ident, $field_type:ident) => (
        impl<TMapping> $field_type<TMapping> for $wrapper_ty<TMapping> where
        TMapping: $mapping_ty { }

        impl_mapping_type!(String, $wrapper_ty, $mapping_ty);

        impl<TMapping> AsRef<str> for $wrapper_ty<TMapping> where
        TMapping: $mapping_ty {
            fn as_ref(&self) -> &str {
                &self.value
            }
        }

        impl<'a, TMapping> PartialEq<&'a str> for $wrapper_ty<TMapping> where
        TMapping: $mapping_ty {
            fn eq(&self, other: & &'a str) -> bool {
                PartialEq::eq(&self.value, *other)
            }

            fn ne(&self, other: & &'a str) -> bool {
                PartialEq::ne(&self.value, *other)
            }
        }

        impl<'a, TMapping> PartialEq<$wrapper_ty<TMapping>> for &'a str where
        TMapping: $mapping_ty {
            fn eq(&self, other: &$wrapper_ty<TMapping>) -> bool {
                PartialEq::eq(*self, &other.value)
            }

            fn ne(&self, other: &$wrapper_ty<TMapping>) -> bool {
                PartialEq::ne(*self, &other.value)
            }
        }

        impl<TMapping> Serialize for $wrapper_ty<TMapping> where
        TMapping: $mapping_ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: Serializer {
                serializer.serialize_str(&self.value)
            }
        }

        impl<'de, TMapping> Deserialize<'de> for $wrapper_ty<TMapping> where
        TMapping: $mapping_ty {
            fn deserialize<D>(deserializer: D) -> Result<$wrapper_ty<TMapping>, D::Error> where
            D: Deserializer<'de> {
                #[derive(Default)]
                struct StringVisitor<TMapping> {
                    _m: PhantomData<TMapping>
                }

                impl<'de, TMapping> Visitor<'de> for StringVisitor<TMapping> where
                TMapping: $mapping_ty {
                    type Value = $wrapper_ty<TMapping>;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
                    {
                        write!(formatter, "a json string")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<$wrapper_ty<TMapping>, E> where
                    E: Error {
                        Ok($wrapper_ty::new(v))
                    }
                }

                deserializer.deserialize_any(StringVisitor::default())
            }
        }
    );
}
