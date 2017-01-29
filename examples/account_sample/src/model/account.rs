use serde::{Serialize, Serializer, Deserialize, Deserializer};
use elastic::types::prelude::{FieldType, Text, DefaultTextMapping, TextMapping, Keyword, DefaultKeywordMapping, KeywordFormat};

#[derive(Serialize, Deserialize, ElasticType)]
pub struct Account {
    account_number: i32,
    balance: i32,
    firstname: FirstName,
    lastname: LastName,
    age: i8,
    gender: Gender,
    address: Address,
    employer: Employer,
    email: Email,
    city: City,
    state: State
}

pub type Address = Text<DefaultTextMapping>;
pub type City = Keyword<DefaultKeywordMapping>;
pub type Employer = Keyword<DefaultKeywordMapping>;
pub type FirstName = Keyword<DefaultKeywordMapping>;
pub type LastName = Keyword<DefaultKeywordMapping>;
pub type State = Keyword<DefaultKeywordMapping>;

pub enum Gender {
    Female,
    Male,
}

impl Serialize for Gender {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> 
        where S: Serializer
    {
        match *self {
            Gender::Female => serializer.serialize_str("F"),
            Gender::Male => serializer.serialize_str("M")
        }
    }
}

impl Deserialize for Gender {
    fn deserialize<D>(_deserializer: &mut D) -> Result<Self, D::Error> 
        where D: Deserializer
    {
        unimplemented!();
    }
}

impl FieldType<DefaultKeywordMapping, KeywordFormat> for Gender { }

pub type Email = Text<EmailMapping>;

#[derive(Default)]
pub struct EmailMapping;
impl TextMapping for EmailMapping {
    fn analyzer() -> Option<&'static str> { Some("email") }
}