// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f localredirectpolicies.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
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

/// Spec is the desired behavior of the local redirect policy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "cilium.io",
    version = "v2",
    kind = "CiliumLocalRedirectPolicy",
    plural = "ciliumlocalredirectpolicies"
)]
#[kube(namespaced)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct CiliumLocalRedirectPolicySpec {
    /// Description can be used by the creator of the policy to describe the purpose of this policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub description: Option<String>,
    /// RedirectBackend specifies backend configuration to redirect traffic to. It can not be empty.
    #[serde(rename = "redirectBackend")]
    pub redirect_backend: CiliumLocalRedirectPolicyRedirectBackend,
    /// RedirectFrontend specifies frontend configuration to redirect traffic from. It can not be empty.
    #[serde(rename = "redirectFrontend")]
    pub redirect_frontend: CiliumLocalRedirectPolicyRedirectFrontend,
}

/// RedirectBackend specifies backend configuration to redirect traffic to. It can not be empty.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumLocalRedirectPolicyRedirectBackend {
    /// LocalEndpointSelector selects node local pod(s) where traffic is redirected to.
    #[serde(rename = "localEndpointSelector")]
    pub local_endpoint_selector: CiliumLocalRedirectPolicyRedirectBackendLocalEndpointSelector,
    /// ToPorts is a list of L4 ports with protocol of node local pod(s) where traffic is redirected to. When multiple ports are specified, the ports must be named.
    #[serde(rename = "toPorts")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub to_ports: Vec<CiliumLocalRedirectPolicyRedirectBackendToPorts>,
}

/// LocalEndpointSelector selects node local pod(s) where traffic is redirected to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumLocalRedirectPolicyRedirectBackendLocalEndpointSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchExpressions"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub match_expressions:
        Option<Vec<CiliumLocalRedirectPolicyRedirectBackendLocalEndpointSelectorMatchExpressions>>,
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
pub struct CiliumLocalRedirectPolicyRedirectBackendLocalEndpointSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator:
        CiliumLocalRedirectPolicyRedirectBackendLocalEndpointSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CiliumLocalRedirectPolicyRedirectBackendLocalEndpointSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// PortInfo specifies L4 port number and name along with the transport protocol
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumLocalRedirectPolicyRedirectBackendToPorts {
    /// Name is a port name, which must contain at least one [a-z], and may also contain [0-9] and '-' anywhere except adjacent to another '-' or in the beginning or the end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub name: Option<String>,
    /// Port is an L4 port number. The string will be strictly parsed as a single uint16.
    pub port: String,
    /// Protocol is the L4 protocol. Accepted values: "TCP", "UDP"
    pub protocol: CiliumLocalRedirectPolicyRedirectBackendToPortsProtocol,
}

/// PortInfo specifies L4 port number and name along with the transport protocol
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CiliumLocalRedirectPolicyRedirectBackendToPortsProtocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

/// RedirectFrontend specifies frontend configuration to redirect traffic from. It can not be empty.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumLocalRedirectPolicyRedirectFrontend {
    /// AddressMatcher is a tuple {IP, port, protocol} that matches traffic to be redirected.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addressMatcher"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub address_matcher: Option<CiliumLocalRedirectPolicyRedirectFrontendAddressMatcher>,
    /// ServiceMatcher specifies Kubernetes service and port that matches traffic to be redirected.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "serviceMatcher"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub service_matcher: Option<CiliumLocalRedirectPolicyRedirectFrontendServiceMatcher>,
}

/// AddressMatcher is a tuple {IP, port, protocol} that matches traffic to be redirected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumLocalRedirectPolicyRedirectFrontendAddressMatcher {
    /// IP is a destination ip address for traffic to be redirected.
    ///  Example: When it is set to "169.254.169.254", traffic destined to "169.254.169.254" is redirected.
    pub ip: String,
    /// ToPorts is a list of destination L4 ports with protocol for traffic to be redirected. When multiple ports are specified, the ports must be named.
    ///  Example: When set to Port: "53" and Protocol: UDP, traffic destined to port '53' with UDP protocol is redirected.
    #[serde(rename = "toPorts")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub to_ports: Vec<CiliumLocalRedirectPolicyRedirectFrontendAddressMatcherToPorts>,
}

/// PortInfo specifies L4 port number and name along with the transport protocol
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumLocalRedirectPolicyRedirectFrontendAddressMatcherToPorts {
    /// Name is a port name, which must contain at least one [a-z], and may also contain [0-9] and '-' anywhere except adjacent to another '-' or in the beginning or the end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub name: Option<String>,
    /// Port is an L4 port number. The string will be strictly parsed as a single uint16.
    pub port: String,
    /// Protocol is the L4 protocol. Accepted values: "TCP", "UDP"
    pub protocol: CiliumLocalRedirectPolicyRedirectFrontendAddressMatcherToPortsProtocol,
}

/// PortInfo specifies L4 port number and name along with the transport protocol
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CiliumLocalRedirectPolicyRedirectFrontendAddressMatcherToPortsProtocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

/// ServiceMatcher specifies Kubernetes service and port that matches traffic to be redirected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumLocalRedirectPolicyRedirectFrontendServiceMatcher {
    /// Namespace is the Kubernetes service namespace. The service namespace must match the namespace of the parent Local Redirect Policy.  For Cluster-wide Local Redirect Policy, this can be any namespace.
    pub namespace: String,
    /// Name is the name of a destination Kubernetes service that identifies traffic to be redirected. The service type needs to be ClusterIP.
    ///  Example: When this field is populated with 'serviceName:myService', all the traffic destined to the cluster IP of this service at the (specified) service port(s) will be redirected.
    #[serde(rename = "serviceName")]
    pub service_name: String,
    /// ToPorts is a list of destination service L4 ports with protocol for traffic to be redirected. If not specified, traffic for all the service ports will be redirected. When multiple ports are specified, the ports must be named.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toPorts")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub to_ports: Option<Vec<CiliumLocalRedirectPolicyRedirectFrontendServiceMatcherToPorts>>,
}

/// PortInfo specifies L4 port number and name along with the transport protocol
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumLocalRedirectPolicyRedirectFrontendServiceMatcherToPorts {
    /// Name is a port name, which must contain at least one [a-z], and may also contain [0-9] and '-' anywhere except adjacent to another '-' or in the beginning or the end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub name: Option<String>,
    /// Port is an L4 port number. The string will be strictly parsed as a single uint16.
    pub port: String,
    /// Protocol is the L4 protocol. Accepted values: "TCP", "UDP"
    pub protocol: CiliumLocalRedirectPolicyRedirectFrontendServiceMatcherToPortsProtocol,
}

/// PortInfo specifies L4 port number and name along with the transport protocol
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CiliumLocalRedirectPolicyRedirectFrontendServiceMatcherToPortsProtocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

/// Status is the most recent status of the local redirect policy. It is a read-only field.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumLocalRedirectPolicyStatus {
    /// TODO Define status(aditi)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ok: Option<bool>,
}
