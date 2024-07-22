// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f cidrgroups.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
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

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "cilium.io",
    version = "v2alpha1",
    kind = "CiliumCIDRGroup",
    plural = "ciliumcidrgroups"
)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct CiliumCIDRGroupSpec {
    /// ExternalCIDRs is a list of CIDRs selecting peers outside the clusters.
    #[serde(rename = "externalCIDRs")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub external_cid_rs: Vec<String>,
}
