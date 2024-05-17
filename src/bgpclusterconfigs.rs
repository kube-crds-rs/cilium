// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f bgpclusterconfigs.yml --schema=derived --docs -b --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube_derive::CustomResource;
    #[cfg(feature = "schemars")]
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
    #[cfg(feature = "builder")]
    pub use typed_builder::TypedBuilder;
}
use self::prelude::*;

/// Spec defines the desired cluster configuration of the BGP control plane.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "cilium.io",
    version = "v2alpha1",
    kind = "CiliumBGPClusterConfig",
    plural = "ciliumbgpclusterconfigs"
)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct CiliumBGPClusterConfigSpec {
    /// A list of CiliumBGPInstance(s) which instructs the BGP control plane how to instantiate virtual BGP routers.
    #[serde(rename = "bgpInstances")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub bgp_instances: Vec<CiliumBGPClusterConfigBgpInstances>,
    /// NodeSelector selects a group of nodes where this BGP Cluster config applies. If empty / nil this config applies to all nodes.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nodeSelector"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub node_selector: Option<CiliumBGPClusterConfigNodeSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPClusterConfigBgpInstances {
    /// LocalASN is the ASN of this BGP instance. Supports extended 32bit ASNs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localASN")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub local_asn: Option<i64>,
    /// Name is the name of the BGP instance. It is a unique identifier for the BGP instance within the cluster configuration.
    pub name: String,
    /// Peers is a list of neighboring BGP peers for this virtual router
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub peers: Option<Vec<CiliumBGPClusterConfigBgpInstancesPeers>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPClusterConfigBgpInstancesPeers {
    /// Name is the name of the BGP peer. It is a unique identifier for the peer within the BGP instance.
    pub name: String,
    /// PeerASN is the ASN of the peer BGP router. Supports extended 32bit ASNs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerASN")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub peer_asn: Option<i64>,
    /// PeerAddress is the IP address of the neighbor. Supports IPv4 and IPv6 addresses.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "peerAddress"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub peer_address: Option<String>,
    /// PeerConfigRef is a reference to a peer configuration resource. If not specified, the default BGP configuration is used for this peer.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "peerConfigRef"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub peer_config_ref: Option<CiliumBGPClusterConfigBgpInstancesPeersPeerConfigRef>,
}

/// PeerConfigRef is a reference to a peer configuration resource. If not specified, the default BGP configuration is used for this peer.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPClusterConfigBgpInstancesPeersPeerConfigRef {
    /// Group is the group of the peer config resource. If not specified, the default of "cilium.io" is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub group: Option<String>,
    /// Kind is the kind of the peer config resource. If not specified, the default of "CiliumBGPPeerConfig" is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub kind: Option<String>,
    /// Name is the name of the peer config resource. Name refers to the name of a Kubernetes object (typically a CiliumBGPPeerConfig).
    pub name: String,
}

/// NodeSelector selects a group of nodes where this BGP Cluster config applies. If empty / nil this config applies to all nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPClusterConfigNodeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchExpressions"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub match_expressions: Option<Vec<CiliumBGPClusterConfigNodeSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchLabels"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPClusterConfigNodeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumBGPClusterConfigNodeSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CiliumBGPClusterConfigNodeSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}
