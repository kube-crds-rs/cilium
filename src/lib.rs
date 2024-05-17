//! Kubernetes CRDs for Cilium
//! 
//! This library provides automatically generated types for the various CRDs included in [Cilium]. It is intended to be used with the [Kube-rs] library.
//! 
//! [Cilium]: https://cilium.io/
//! [Kube-rs]: https://kube.rs/

pub mod clusterwideenvoyconfigs;
pub use clusterwideenvoyconfigs::*;
pub mod clusterwidenetworkpolicies;
pub use clusterwidenetworkpolicies::*;
pub mod egressgatewaypolicies;
pub use egressgatewaypolicies::*;
pub mod endpoints;
pub use endpoints::*;
pub mod envoyconfigs;
pub use envoyconfigs::*;
pub mod externalworkloads;
pub use externalworkloads::*;
pub mod identities;
pub use identities::*;
pub mod localredirectpolicies;
pub use localredirectpolicies::*;
pub mod networkpolicies;
pub use networkpolicies::*;
pub mod nodes;
pub use nodes::*;
#[cfg(feature = "alpha")]
pub mod bgpadvertisements;
#[cfg(feature = "alpha")]
pub use bgpadvertisements::*;
#[cfg(feature = "alpha")]
pub mod bgpclusterconfigs;
#[cfg(feature = "alpha")]
pub use bgpclusterconfigs::*;
#[cfg(feature = "alpha")]
pub mod bgpnodeconfigoverrides;
#[cfg(feature = "alpha")]
pub use bgpnodeconfigoverrides::*;
#[cfg(feature = "alpha")]
pub mod bgpnodeconfigs;
#[cfg(feature = "alpha")]
pub use bgpnodeconfigs::*;
#[cfg(feature = "alpha")]
pub mod bgppeerconfigs;
#[cfg(feature = "alpha")]
pub use bgppeerconfigs::*;
#[cfg(feature = "alpha")]
pub mod bgppeeringpolicies;
#[cfg(feature = "alpha")]
pub use bgppeeringpolicies::*;
#[cfg(feature = "alpha")]
pub mod cidrgroups;
#[cfg(feature = "alpha")]
pub use cidrgroups::*;
#[cfg(feature = "alpha")]
pub mod endpointslices;
#[cfg(feature = "alpha")]
pub use endpointslices::*;
#[cfg(feature = "alpha")]
pub mod l2announcementpolicies;
#[cfg(feature = "alpha")]
pub use l2announcementpolicies::*;
#[cfg(feature = "alpha")]
pub mod loadbalancerippools;
#[cfg(feature = "alpha")]
pub use loadbalancerippools::*;
#[cfg(feature = "alpha")]
pub mod podippools;
#[cfg(feature = "alpha")]
pub use podippools::*;
#[cfg(feature = "alpha")]
pub mod nodeconfigs;
#[cfg(feature = "alpha")]
pub use nodeconfigs::*;
