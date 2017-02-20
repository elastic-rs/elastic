#![feature(plugin, test)]
#![plugin(json_str, elastic_date_macros)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate elastic_types_derive;

#[allow(plugin_as_library)]
#[macro_use]
extern crate json_str;

extern crate serde;
extern crate serde_json;
pub extern crate chrono;
extern crate geo as georust;
extern crate geojson;
extern crate test;

#[macro_use]
extern crate elastic_types;

pub mod date_fixtures {
    use elastic_types::prelude::*;

    #[derive(Default, Clone)]
    pub struct MyDateMapping;
    impl DateMapping for MyDateMapping {
        type Format = EpochMillis;

        fn null_value() -> Option<Date<Self::Format, Self>> {
            Some(Date::build(2015, 3, 14, 16, 45, 13, 778))
        }

        fn boost() -> Option<f32>                 { Some(1.01) }

        fn index() -> Option<bool>                 { Some(true) }

        fn doc_values() -> Option<bool>         { Some(true) }

        fn include_in_all() -> Option<bool>     { Some(false) }

        fn store() -> Option<bool>                 { Some(true) }

        fn ignore_malformed() -> Option<bool>     { Some(true) }
    }
}

pub mod string_fixtures {
    use std::collections::BTreeMap;
    use elastic_types::prelude::*;

    #[derive(Default, Clone)]
    pub struct MyTextMapping;
    impl TextMapping for MyTextMapping {
        fn fields() -> Option<BTreeMap<&'static str, StringField>> {
            let mut fields = BTreeMap::new();

            fields.insert("raw", StringField::Keyword(
                KeywordFieldMapping {
                    analyzer: Some("my_analyzer"),
                    ..Default::default()
                })
            );

            fields.insert("count", StringField::TokenCount(
                ElasticTokenCountFieldMapping::default())
            );

            fields.insert("comp", StringField::Completion(
                ElasticCompletionFieldMapping::default())
            );

            Some(fields)
        }

        fn fielddata_frequency_filter() -> Option<FieldDataFrequencyFilter> { 
            Some(FieldDataFrequencyFilter { min: Some(0.0), ..Default::default() })
        }

        fn analyzer() -> Option<&'static str>                 { Some("my_analyzer") }

        fn boost() -> Option<f32>                             { Some(1.3) }

        fn eager_global_ordinals() -> Option<bool>             { Some(false) }

        fn fielddata() -> Option<bool>                         { Some(true) }

        fn include_in_all() -> Option<bool>                 { Some(true) }

        fn ignore_above() -> Option<u32>                     { Some(512) }

        fn index() -> Option<bool>                             { Some(false) }

        fn index_options() -> Option<IndexOptions>             { Some(IndexOptions::Freqs) }

        fn norms() -> Option<bool>                             { Some(true) }

        fn position_increment_gap() -> Option<u32>             { Some(1) }

        fn store() -> Option<bool>                             { Some(true) }

        fn search_analyzer() -> Option<&'static str>         { Some("my_analyzer") }

        fn search_quote_analyzer() -> Option<&'static str>     { Some("my_analyzer") }

        fn similarity() -> Option<&'static str>             { Some("BM25") }

        fn term_vector() -> Option<TermVector>                 { Some(TermVector::Yes) }
    }

    #[derive(Default, Clone)]
    pub struct MyKeywordMapping;
    impl KeywordMapping for MyKeywordMapping {
        fn fields() -> Option<BTreeMap<&'static str, StringField>> {
            let mut fields = BTreeMap::new();

            fields.insert("text", StringField::Text(
                TextFieldMapping {
                    analyzer: Some("my_analyzer"),
                    ..Default::default()
                })
            );

            fields.insert("count", StringField::TokenCount(
                ElasticTokenCountFieldMapping::default())
            );

            fields.insert("comp", StringField::Completion(
                ElasticCompletionFieldMapping::default())
            );

            Some(fields)
        }

        fn analyzer() -> Option<&'static str>             { Some("my_analyzer") }

        fn boost() -> Option<f32>                         { Some(1.03) }

        fn doc_values() -> Option<bool>                 { Some(true) }

        fn eager_global_ordinals() -> Option<bool>         { Some(false) }

        fn include_in_all() -> Option<bool>             { Some(false) }

        fn ignore_above() -> Option<u32>                 { Some(256) }

        fn index() -> Option<bool>                         { Some(true) }

        fn index_options() -> Option<IndexOptions>         { Some(IndexOptions::Docs) }

        fn norms() -> Option<bool>                         { Some(false) }

        fn null_value() -> Option<&'static str>         { Some("my string") }

        fn store() -> Option<bool>                         { Some(false) }

        fn search_analyzer() -> Option<&'static str>     { Some("my_analyzer") }

        fn similarity() -> Option<&'static str>         { Some("classic") }
    }
}

pub mod boolean_fixtures {
    use elastic_types::prelude::*;

    #[derive(Default, Clone)]
    pub struct MyBooleanMapping;
    impl BooleanMapping for MyBooleanMapping {
        fn boost() -> Option<f32>             { Some(1.01) }

        fn index() -> Option<bool>             { Some(false) }

        fn doc_values() -> Option<bool>     { Some(true) }

        fn store() -> Option<bool>             { Some(true) }

        fn null_value() -> Option<bool>     { Some(false) }
    }
}

pub mod number_fixtures {
    use elastic_types::prelude::*;

    #[derive(Default, Clone)]
    pub struct MyIntegerMapping;
    impl IntegerMapping for MyIntegerMapping {
        fn coerce() -> Option<bool>             { Some(true) }

        fn boost() -> Option<f32>                 { Some(1.1) }

        fn doc_values() -> Option<bool>         { Some(false) }

        fn ignore_malformed() -> Option<bool>     { Some(true) }

        fn include_in_all() -> Option<bool>     { Some(true) }

        fn index() -> Option<bool>                 { Some(false) }

        fn store() -> Option<bool>                 { Some(true) }

        fn null_value() -> Option<i32>             { Some(42) }
    }

    #[derive(Default, Clone)]
    pub struct MyLongMapping;
    impl LongMapping for MyLongMapping {
        fn coerce() -> Option<bool>             { Some(true) }

        fn boost() -> Option<f32>                 { Some(1.1) }

        fn doc_values() -> Option<bool>         { Some(false) }

        fn ignore_malformed() -> Option<bool>     { Some(true) }

        fn include_in_all() -> Option<bool>     { Some(true) }

        fn index() -> Option<bool>                 { Some(false) }

        fn store() -> Option<bool>                 { Some(true) }

        fn null_value() -> Option<i64>             { Some(-42) }
    }

    #[derive(Default, Clone)]
    pub struct MyShortMapping;
    impl ShortMapping for MyShortMapping {
        fn coerce() -> Option<bool>             { Some(true) }

        fn boost() -> Option<f32>                 { Some(1.1) }

        fn doc_values() -> Option<bool>         { Some(false) }

        fn ignore_malformed() -> Option<bool>     { Some(true) }

        fn include_in_all() -> Option<bool>     { Some(true) }

        fn index() -> Option<bool>                 { Some(false) }

        fn store() -> Option<bool>                 { Some(true) }

        fn null_value() -> Option<i16>             { Some(42) }
    }

    #[derive(Default, Clone)]
    pub struct MyByteMapping;
    impl ByteMapping for MyByteMapping {
        fn coerce() -> Option<bool>             { Some(true) }

        fn boost() -> Option<f32>                 { Some(1.1) }

        fn doc_values() -> Option<bool>         { Some(false) }

        fn ignore_malformed() -> Option<bool>     { Some(true) }

        fn include_in_all() -> Option<bool>     { Some(true) }

        fn index() -> Option<bool>                 { Some(false) }

        fn store() -> Option<bool>                 { Some(true) }

        fn null_value() -> Option<i8>             { Some(1) }
    }

    #[derive(Default, Clone)]
    pub struct MyFloatMapping;
    impl FloatMapping for MyFloatMapping {
        fn coerce() -> Option<bool>             { Some(true) }

        fn boost() -> Option<f32>                 { Some(1.1) }

        fn doc_values() -> Option<bool>         { Some(false) }

        fn ignore_malformed() -> Option<bool>     { Some(true) }

        fn include_in_all() -> Option<bool>     { Some(true) }

        fn index() -> Option<bool>                 { Some(false) }

        fn store() -> Option<bool>                 { Some(true) }

        fn null_value() -> Option<f32>             { Some(1.04) }
    }

    #[derive(Default, Clone)]
    pub struct MyDoubleMapping;
    impl DoubleMapping for MyDoubleMapping {
        fn coerce() -> Option<bool>             { Some(true) }

        fn boost() -> Option<f32>                 { Some(1.1) }

        fn doc_values() -> Option<bool>         { Some(false) }

        fn ignore_malformed() -> Option<bool>     { Some(true) }

        fn include_in_all() -> Option<bool>     { Some(true) }

        fn index() -> Option<bool>                 { Some(false) }

        fn store() -> Option<bool>                 { Some(true) }

        fn null_value() -> Option<f64>             { Some(-0.00002) }
    }
}

pub mod ip_fixtures {
    use std::net::Ipv4Addr;
    use elastic_types::prelude::*;

    #[derive(Default, Clone)]
    pub struct MyIpMapping;
    impl IpMapping for MyIpMapping {
        fn boost() -> Option<f32>                 { Some(1.01) }

        fn index() -> Option<bool>                 { Some(false) }

        fn doc_values() -> Option<bool>         { Some(true) }

        fn store() -> Option<bool>                 { Some(true) }

        fn null_value() -> Option<Ipv4Addr>     { Some(Ipv4Addr::new(127, 0, 0, 1)) }    
    }
}

pub mod geo_point_fixtures {
    use elastic_types::prelude::*;

    #[derive(Default, Clone)]
    pub struct MyGeoPointMapping;
    impl GeoPointMapping for MyGeoPointMapping {
        type Format = GeoPointArray;

        fn geohash() -> Option<bool>                 { Some(false) }

        fn geohash_precision() -> Option<Distance>     { Some(Distance(50.0, DistanceUnit::Meters)) }

        fn geohash_prefix() -> Option<bool>         { Some(true) }

        fn ignore_malformed() -> Option<bool>         { Some(true) }

        fn lat_lon() -> Option<bool>                 { Some(true) }
    }
}

pub mod geo_shape_fixtures {
    use elastic_types::prelude::*;

    #[derive(Default, Clone)]
    pub struct MyGeoShapeMapping;
    impl GeoShapeMapping for MyGeoShapeMapping {
        fn tree() -> Option<Tree>                     { Some(Tree::Geohash) }

        fn precision() -> Option<Distance>             { Some(Distance(50.0, DistanceUnit::Meters)) }

        fn tree_levels() -> Option<i32>             { Some(8) }

        fn strategy() -> Option<Strategy>             { Some(Strategy::Recursive) }

        fn distance_error_pct() -> Option<f32>         { Some(0.5) }

        fn orientation() -> Option<Orientation>     { Some(Orientation::Clockwise) }

        fn points_only() -> Option<bool>             { Some(false) }
    }
}

pub mod object_fixtures {
    use serde::ser::SerializeStruct;
    use chrono::{ DateTime, UTC };
    use elastic_types::prelude::*;

    #[derive(Default, Clone)]
    pub struct MySmlMapping;
    impl DocumentMapping for MySmlMapping {
        fn name() -> &'static str { "ty" }
    }
    impl PropertiesMapping for MySmlMapping {
        fn props_len() -> usize { 3 }
        
        fn serialize_props<S>(state: &mut S) -> Result<(), S::Error>
        where S: SerializeStruct {
            try!(field_ser(state, "integer", i32::mapping()));
            try!(field_ser(state, "string", String::mapping()));
            try!(field_ser(state, "date", DateTime::<UTC>::mapping()));

            Ok(())
        }
    }

    #[derive(Default, Clone)]
    pub struct MyMedMapping;
    impl DocumentMapping for MyMedMapping {
        fn name() -> &'static str { "ty" }
    }
    impl PropertiesMapping for MyMedMapping {
        fn props_len() -> usize { 4 }
        
        fn serialize_props<S>(state: &mut S) -> Result<(), S::Error>
        where S: SerializeStruct {
            try!(field_ser(state, "integer", i32::mapping()));
            try!(field_ser(state, "string", String::mapping()));
            try!(field_ser(state, "date", DateTime::<UTC>::mapping()));
            try!(field_ser(state, "field", MySmlMapping));

            Ok(())
        }
    }

    #[derive(Default, Clone)]
    pub struct MyLrgMapping;
    impl DocumentMapping for MyLrgMapping {
        fn name() -> &'static str { "ty" }
    }
    impl PropertiesMapping for MyLrgMapping {
        fn props_len() -> usize { 4 }
        
        fn serialize_props<S>(state: &mut S) -> Result<(), S::Error>
        where S: SerializeStruct {
            try!(field_ser(state, "integer", i32::mapping()));
            try!(field_ser(state, "string", String::mapping()));
            try!(field_ser(state, "date", DateTime::<UTC>::mapping()));
            try!(field_ser(state, "field", MyMedMapping));

            Ok(())
        }
    }
}

pub mod object;
pub mod geo_point;
pub mod geo_shape;
pub mod date;
pub mod string;
pub mod number;
pub mod boolean;