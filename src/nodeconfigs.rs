// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f /tmp/tmp6lzn_9qr --schema=derived --docs -b --derive=Default --derive=PartialEq
// kopium version: 0.20.1

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

/// Spec is the desired Cilium configuration overrides for a given node
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "cilium.io",
    version = "v2alpha1",
    kind = "CiliumNodeConfig",
    plural = "ciliumnodeconfigs"
)]
#[kube(namespaced)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct CiliumNodeConfigSpec {
    /// Defaults is treated the same as the cilium-config ConfigMap - a set of key-value pairs parsed by the agent and operator processes. Each key must be a valid config-map data field (i.e. a-z, A-Z, -, _, and .)
    #[cfg_attr(feature = "builder", builder(default))]
    pub defaults: BTreeMap<String, String>,
    /// NodeSelector is a label selector that determines to which nodes this configuration applies. If not supplied, then this config applies to no nodes. If empty, then it applies to all nodes.
    #[serde(rename = "nodeSelector")]
    pub node_selector: CiliumNodeConfigNodeSelector,
}

/// NodeSelector is a label selector that determines to which nodes this configuration applies. If not supplied, then this config applies to no nodes. If empty, then it applies to all nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumNodeConfigNodeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchExpressions"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub match_expressions: Option<Vec<CiliumNodeConfigNodeSelectorMatchExpressions>>,
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumNodeConfigNodeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub values: Option<Vec<String>>,
}
