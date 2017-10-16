/*! Contains the `NodesInfoResponse` type for sniffing node addresses in the cluster. */

use std::fmt;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use client::responses::parse::{IsOk, HttpResponseHead, Unbuffered, MaybeOkResponse, ResponseBody, ParseResponseError};

use std::iter::IntoIterator;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Deserialize)]
pub struct NodesInfoResponse {
    #[serde(deserialize_with = "deserialize_nodes")]
    nodes: Vec<SniffedNode>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SniffedNode {
    pub http: Option<SniffedNodeHttp>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SniffedNodeHttp {
    pub publish_address: Option<String>,
}

impl IntoIterator for NodesInfoResponse {
    type Item = SniffedNode;
    type IntoIter = IntoIter<SniffedNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl IsOk for NodesInfoResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, unbuffered: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(unbuffered)),
            _ => Ok(MaybeOkResponse::err(unbuffered)),
        }
    }
}

fn deserialize_nodes<'de, D>(deserializer: D) -> Result<Vec<SniffedNode>, D::Error>
    where
        D: Deserializer<'de>
{
    #[derive(Debug, PartialEq)]
    struct SniffedNodeSet(Vec<SniffedNode>);

    impl<'de> Deserialize<'de> for SniffedNodeSet {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
        {
            #[derive(Default)]
            struct SniffedNodeSetVisitor;

            impl<'de> Visitor<'de> for SniffedNodeSetVisitor {
                type Value = SniffedNodeSet;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a map of node values")
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                    where M: MapAccess<'de>
                {
                    let mut nodes = Vec::with_capacity(access.size_hint().unwrap_or(0));

                    while let Some((_, node)) = access.next_entry::<&str, _>()? {
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

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    #[test]
    fn deserialise_nodes() {
        let nodes = json!({
            "nodes": {
                "node1": {
                    "http": {
                        "publish_address": "1.1.1.1:9200"
                    }
                },
                "node2": {
                    "http": {
                        "publish_address": "1.1.1.2:9200"
                    }
                }
            }
        }).to_string();

        let expected = NodesInfoResponse {
            nodes: vec![
                SniffedNode {
                    http: Some(SniffedNodeHttp {
                        publish_address: Some("1.1.1.1:9200".to_owned()),
                    }),
                },
                 SniffedNode {
                    http: Some(SniffedNodeHttp {
                        publish_address: Some("1.1.1.2:9200".to_owned()),
                    }),
                }
            ]
        };

        let actual: NodesInfoResponse = serde_json::from_str(&nodes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialise_nodes_empty() {
        let nodes = json!({
            "nodes": { }
        }).to_string();

        let expected = NodesInfoResponse {
            nodes: vec![]
        };

        let actual: NodesInfoResponse = serde_json::from_str(&nodes).unwrap();

        assert_eq!(expected, actual);
    }
}
