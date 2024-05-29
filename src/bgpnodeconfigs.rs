// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f /tmp/tmpw4epqy40 --schema=derived --docs -b --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube_derive::CustomResource;
    #[cfg(feature = "schemars")]
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    #[cfg(feature = "builder")]
    pub use typed_builder::TypedBuilder;
}
use self::prelude::*;

/// Spec is the specification of the desired behavior of the CiliumBGPNodeConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "cilium.io",
    version = "v2alpha1",
    kind = "CiliumBGPNodeConfig",
    plural = "ciliumbgpnodeconfigs"
)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct CiliumBGPNodeConfigSpec {
    /// BGPInstances is a list of BGP router instances on the node.
    #[serde(rename = "bgpInstances")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub bgp_instances: Vec<CiliumBGPNodeConfigBgpInstances>,
}

/// CiliumBGPNodeInstance is a single BGP router instance configuration on the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPNodeConfigBgpInstances {
    /// LocalASN is the ASN of this virtual router. Supports extended 32bit ASNs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localASN")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub local_asn: Option<i64>,
    /// LocalPort is the port on which the BGP daemon listens for incoming connections.
    ///  If not specified, BGP instance will not listen for incoming connections.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPort")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub local_port: Option<i32>,
    /// Name is the name of the BGP instance. This name is used to identify the BGP instance on the node.
    pub name: String,
    /// Peers is a list of neighboring BGP peers for this virtual router
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub peers: Option<Vec<CiliumBGPNodeConfigBgpInstancesPeers>>,
    /// RouterID is the BGP router ID of this virtual router. This configuration is derived from CiliumBGPNodeConfigOverride resource.
    ///  If not specified, the router ID will be derived from the node local address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routerID")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub router_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPNodeConfigBgpInstancesPeers {
    /// LocalAddress is the IP address of the local interface to use for the peering session. This configuration is derived from CiliumBGPNodeConfigOverride resource. If not specified, the local address will be used for setting up peering.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "localAddress"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub local_address: Option<String>,
    /// Name is the name of the BGP peer. This name is used to identify the BGP peer for the BGP instance.
    pub name: String,
    /// PeerASN is the ASN of the peer BGP router. Supports extended 32bit ASNs
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
    pub peer_config_ref: Option<CiliumBGPNodeConfigBgpInstancesPeersPeerConfigRef>,
}

/// PeerConfigRef is a reference to a peer configuration resource. If not specified, the default BGP configuration is used for this peer.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPNodeConfigBgpInstancesPeersPeerConfigRef {
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

/// Status is the most recently observed status of the CiliumBGPNodeConfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPNodeConfigStatus {
    /// BGPInstances is the status of the BGP instances on the node.
    #[serde(rename = "bgpInstances")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub bgp_instances: Vec<CiliumBGPNodeConfigStatusBgpInstances>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPNodeConfigStatusBgpInstances {
    /// LocalASN is the ASN of this BGP instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localASN")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub local_asn: Option<i64>,
    /// Name is the name of the BGP instance. This name is used to identify the BGP instance on the node.
    pub name: String,
    /// PeerStatuses is the state of the BGP peers for this BGP instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub peers: Option<Vec<CiliumBGPNodeConfigStatusBgpInstancesPeers>>,
}

/// CiliumBGPNodePeerStatus is the status of a BGP peer.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPNodeConfigStatusBgpInstancesPeers {
    /// Name is the name of the BGP peer.
    pub name: String,
    /// PeerASN is the ASN of the neighbor.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "peerASN")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub peer_asn: Option<i64>,
    /// PeerAddress is the IP address of the neighbor.
    #[serde(rename = "peerAddress")]
    pub peer_address: String,
    /// PeeringState is last known state of the peering session.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "peeringState"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub peering_state: Option<String>,
    /// RoutesAdvertised is the number of routes advertised to this peer.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "routesAdvertised"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub routes_advertised: Option<i32>,
    /// RoutesReceived is the number of routes received from this peer.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "routesReceived"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub routes_received: Option<i32>,
    /// Timers is the state of the negotiated BGP timers for this peer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub timers: Option<CiliumBGPNodeConfigStatusBgpInstancesPeersTimers>,
    /// Uptime is the time since the last peering session was established.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub uptime: Option<String>,
}

/// Timers is the state of the negotiated BGP timers for this peer.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CiliumBGPNodeConfigStatusBgpInstancesPeersTimers {
    /// AppliedHoldTimeSeconds is the negotiated hold time for this peer.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "appliedHoldTimeSeconds"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub applied_hold_time_seconds: Option<i32>,
    /// AppliedKeepaliveSeconds is the negotiated keepalive time for this peer.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "appliedKeepaliveSeconds"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub applied_keepalive_seconds: Option<i32>,
}
