/*
This file is automatically included into the generated `genned::params` module
*/

use crate::types::string::{Keyword, keyword::mapping::KeywordMapping};

impl<'a, M> From<Keyword<M>> for Id<'a>
where
    M: KeywordMapping,
{
    fn from(value: Keyword<M>) -> Id<'a> {
        let value: String = value.into();

        Id::from(value)
    }
}

impl<'a, M> From<&'a Keyword<M>> for Id<'a>
where
    M: KeywordMapping,
{
    fn from(value: &'a Keyword<M>) -> Id<'a> {
        let value: &str = value.into();

        Id::from(value)
    }
}

macro_rules! impl_from_num_for_id {
    ($num:ty) => (
        impl<'a> From<$num> for Id<'a> {
            fn from(value: $num) -> Id<'a> {
                Id::from(value.to_string())
            }
        }

        impl<'a, 'b> From<&'b $num> for Id<'a> {
            fn from(value: &'b $num) -> Id<'a> {
                Id::from(value.to_string())
            }
        }
    )
}

impl_from_num_for_id!(u32);
impl_from_num_for_id!(u64);
impl_from_num_for_id!(usize);
impl_from_num_for_id!(i32);
impl_from_num_for_id!(i64);
impl_from_num_for_id!(isize);
