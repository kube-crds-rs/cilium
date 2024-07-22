// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f /tmp/tmpsv6okq5p --schema=derived --docs -b --derive=Default --derive=PartialEq
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

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "cilium.io",
    version = "v2",
    kind = "CiliumEgressGatewayPolicy",
    plural = "ciliumegressgatewaypolicies"
)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct CiliumEgressGatewayPolicySpec {
    /// DestinationCIDRs is a list of destination CIDRs for destination IP addresses. If a destination IP matches any one CIDR, it will be selected.
    #[serde(rename = "destinationCIDRs")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub destination_cid_rs: Vec<String>,
    /// EgressGateway is the gateway node responsible for SNATing traffic.
    #[serde(rename = "egressGateway")]
    pub egress_gateway: CiliumEgressGatewayPolicyEgressGateway,
    /// ExcludedCIDRs is a list of destination CIDRs that will be excluded from the egress gateway redirection and SNAT logic. Should be a subset of destinationCIDRs otherwise it will not have any effect.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "excludedCIDRs"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub excluded_cid_rs: Option<Vec<String>>,
    /// Egress represents a list of rules by which egress traffic is filtered from the source pods.
    #[cfg_attr(feature = "builder", builder(default))]
    pub selectors: Vec<CiliumEgressGatewayPolicySelectors>,
}

/// EgressGateway is the gateway node responsible for SNATing traffic.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumEgressGatewayPolicyEgressGateway {
    /// EgressIP is the source IP address that the egress traffic is SNATed with.
    ///  Example: When set to "192.168.1.100", matching egress traffic will be redirected to the node matching the NodeSelector field and SNATed with IP address 192.168.1.100.
    ///  When none of the Interface or EgressIP fields is specified, the policy will use the first IPv4 assigned to the interface with the default route.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "egressIP")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub egress_ip: Option<String>,
    /// Interface is the network interface to which the egress IP address that the traffic is SNATed with is assigned.
    ///  Example: When set to "eth1", matching egress traffic will be redirected to the node matching the NodeSelector field and SNATed with the first IPv4 address assigned to the eth1 interface.
    ///  When none of the Interface or EgressIP fields is specified, the policy will use the first IPv4 assigned to the interface with the default route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub interface: Option<String>,
    /// This is a label selector which selects the node that should act as egress gateway for the given policy. In case multiple nodes are selected, only the first one in the lexical ordering over the node names will be used. This field follows standard label selector semantics.
    #[serde(rename = "nodeSelector")]
    pub node_selector: CiliumEgressGatewayPolicyEgressGatewayNodeSelector,
}

/// This is a label selector which selects the node that should act as egress gateway for the given policy. In case multiple nodes are selected, only the first one in the lexical ordering over the node names will be used. This field follows standard label selector semantics.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumEgressGatewayPolicyEgressGatewayNodeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchExpressions"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub match_expressions:
        Option<Vec<CiliumEgressGatewayPolicyEgressGatewayNodeSelectorMatchExpressions>>,
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
pub struct CiliumEgressGatewayPolicyEgressGatewayNodeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumEgressGatewayPolicyEgressGatewayNodeSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CiliumEgressGatewayPolicyEgressGatewayNodeSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumEgressGatewayPolicySelectors {
    /// Selects Namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "namespaceSelector"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub namespace_selector: Option<CiliumEgressGatewayPolicySelectorsNamespaceSelector>,
    /// This is a label selector which selects Pods. This field follows standard label selector semantics; if present but empty, it selects all pods.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "podSelector"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub pod_selector: Option<CiliumEgressGatewayPolicySelectorsPodSelector>,
}

/// Selects Namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumEgressGatewayPolicySelectorsNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchExpressions"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub match_expressions:
        Option<Vec<CiliumEgressGatewayPolicySelectorsNamespaceSelectorMatchExpressions>>,
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
pub struct CiliumEgressGatewayPolicySelectorsNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumEgressGatewayPolicySelectorsNamespaceSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CiliumEgressGatewayPolicySelectorsNamespaceSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// This is a label selector which selects Pods. This field follows standard label selector semantics; if present but empty, it selects all pods.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumEgressGatewayPolicySelectorsPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchExpressions"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub match_expressions:
        Option<Vec<CiliumEgressGatewayPolicySelectorsPodSelectorMatchExpressions>>,
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
pub struct CiliumEgressGatewayPolicySelectorsPodSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumEgressGatewayPolicySelectorsPodSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CiliumEgressGatewayPolicySelectorsPodSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}
