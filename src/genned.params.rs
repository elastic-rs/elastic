macro_rules! impl_from_num_for_id {
    ($num:ty) => (
        impl<'a> From<$num> for Id<'a> {
            fn from(value: $num) -> Id<'a> {
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