use serde::Serialize;

pub trait SerializeField<F> {
    type Field: Serialize + Default;
}

pub trait FieldMapping<F>
    where Self: Default + SerializeField<F>,
          F: Default
{
    fn data_type() -> &'static str {
        "object"
    }
}
