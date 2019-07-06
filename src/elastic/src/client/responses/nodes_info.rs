/*! Contains the `NodesInfoResponse` type for sniffing node addresses in the cluster. */

use crate::http::receiver::IsOkOnSuccess;
use serde::de::{
    Deserialize,
    Deserializer,
    MapAccess,
    Visitor,
};

use std::{
    fmt,
    iter::IntoIterator,
    slice::Iter,
    vec::IntoIter,
};

/** Response for a [nodes info request](http://www.elastic.co/guide/en/elasticsearch/reference/current/cluster-nodes-info.html). */
#[derive(Debug, PartialEq, Deserialize)]
pub struct NodesInfoResponse {
    #[serde(deserialize_with = "deserialize_nodes")]
    nodes: Vec<SniffedNode>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct SniffedNode {
    http: Option<SniffedNodeHttp>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct SniffedNodeHttp {
    publish_address: Option<String>,
}

impl NodesInfoResponse {
    /** Iterate over borrowed publish addresses in the cluster. */
    pub fn iter_addrs(&self) -> IterAddrs {
        IterAddrs(self.nodes.iter())
    }

    /** Iterate over owned publish addresses in the cluster. */
    pub fn into_iter_addrs(self) -> IntoIterAddrs {
        IntoIterAddrs(self.nodes.into_iter())
    }
}

/**
An iterator over node publish addresses.

This is the result of calling [`NodesInfoResponse.iter_addrs()`](structNodesInfoResponse.html#method.iter_addrs).
*/
pub struct IterAddrs<'a>(Iter<'a, SniffedNode>);

impl<'a> Iterator for IterAddrs<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.0.next() {
            if let Some(addr) = node
                .http
                .as_ref()
                .and_then(|http| http.publish_address.as_ref())
            {
                return Some(addr);
            }
        }

        None
    }
}

/**
An iterator over node publish addresses.

This is the result of calling [`NodesInfoResponse.innto_iter_addrs()`](structNodesInfoResponse.html#method.innto_iter_addrs).
*/
pub struct IntoIterAddrs(IntoIter<SniffedNode>);

impl Iterator for IntoIterAddrs {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.0.next() {
            if let Some(addr) = node.http.and_then(|http| http.publish_address) {
                return Some(addr);
            }
        }

        None
    }
}

impl IsOkOnSuccess for NodesInfoResponse {}

fn deserialize_nodes<'de, D>(deserializer: D) -> Result<Vec<SniffedNode>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, PartialEq)]
    struct SniffedNodeSet(Vec<SniffedNode>);

    impl<'de> Deserialize<'de> for SniffedNodeSet {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            #[derive(Default)]
            struct SniffedNodeSetVisitor;

            impl<'de> Visitor<'de> for SniffedNodeSetVisitor {
                type Value = SniffedNodeSet;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a map of node values")
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    let mut nodes = Vec::with_capacity(access.size_hint().unwrap_or(0));

                    while let Some((_, node)) = access.next_entry::<String, _>()? {
                        nodes.push(node);
                    }

                    Ok(SniffedNodeSet(nodes))
                }
            }

            deserializer.deserialize_map(SniffedNodeSetVisitor::default())
        }
    }

    let nodes = SniffedNodeSet::deserialize(deserializer)?;

    Ok(nodes.0)
}
