#[macro_export]
macro_rules! ser_field {
    ($serializer:ident, $field:expr, $val_opt:expr) => (
        if let Some(f) = $val_opt {
            try!($serializer.serialize_field($field, &f));
        }
    )
}
