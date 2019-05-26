pub(crate) mod common;
pub(crate) mod exists;
pub(crate) mod matchfilter;
pub(crate) mod range;
pub(crate) mod term;
pub(crate) mod wildcard;

pub(crate) use self::{
    exists::ExistsFilter,
    matchfilter::MatchFilter,
    range::RangeFilter,
    term::TermFilter,
    wildcard::WildcardFilter,
};

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum Filters {
    term(TermFilter),
    range(RangeFilter),
    exists(ExistsFilter),
    #[serde(rename = "match")]
    match_(MatchFilter),
    wildcard(WildcardFilter),
}

impl From<RangeFilter> for Filters {
    fn from(r: RangeFilter) -> Self {
        Filters::range(r)
    }
}

impl From<TermFilter> for Filters {
    fn from(t: TermFilter) -> Self {
        Filters::term(t)
    }
}

impl From<ExistsFilter> for Filters {
    fn from(e: ExistsFilter) -> Self {
        Filters::exists(e)
    }
}

impl From<MatchFilter> for Filters {
    fn from(m: MatchFilter) -> Self {
        Filters::match_(m)
    }
}

impl From<WildcardFilter> for Filters {
    fn from(w: WildcardFilter) -> Self {
        Filters::wildcard(w)
    }
}
