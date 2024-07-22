#!/usr/bin/env python3

import yaml
import requests
import tempfile
import subprocess


def pascal_to_snake(s):
    return "".join(["_" + c.lower() if c.isupper() else c for c in s]).lstrip("_")


rust_lib = """//! Kubernetes CRDs for Cilium
//! 
//! This library provides automatically generated types for the various CRDs included in [Cilium]. It is intended to be used with the [Kube-rs] library.
//! 
//! [Cilium]: https://cilium.io/
//! [Kube-rs]: https://kube.rs/

"""


crds_stable = [
    "ciliumclusterwideenvoyconfigs.yaml",
    "ciliumclusterwidenetworkpolicies.yaml",
    "ciliumegressgatewaypolicies.yaml",
    "ciliumendpoints.yaml",
    "ciliumenvoyconfigs.yaml",
    "ciliumexternalworkloads.yaml",
    "ciliumidentities.yaml",
    "ciliumlocalredirectpolicies.yaml",
    "ciliumnetworkpolicies.yaml",
    "ciliumnodes.yaml",
]

crds_alpha = [
    "ciliumbgpadvertisements.yaml",
    "ciliumbgpclusterconfigs.yaml",
    "ciliumbgpnodeconfigoverrides.yaml",
    "ciliumbgpnodeconfigs.yaml",
    "ciliumbgppeerconfigs.yaml",
    "ciliumbgppeeringpolicies.yaml",
    "ciliumcidrgroups.yaml",
    "ciliumendpointslices.yaml",
    "ciliuml2announcementpolicies.yaml",
    "ciliumloadbalancerippools.yaml",
    "ciliumpodippools.yaml",
    "ciliumnodeconfigs.yaml",
]

crds = {}

# Fetch all CRDs
for crd in crds_stable:
    r = requests.get(
        f"https://raw.githubusercontent.com/cilium/cilium/v1.15.7/pkg/k8s/apis/cilium.io/client/crds/v2/{crd}"
    )
    r.raise_for_status()
    all_crds = yaml.safe_load_all(r.text)
    crds[crd] = [crd for crd in all_crds]

for crd in crds_alpha:
    r = requests.get(
        f"https://raw.githubusercontent.com/cilium/cilium/v1.15.7/pkg/k8s/apis/cilium.io/client/crds/v2alpha1/{crd}"
    )
    r.raise_for_status()
    all_crds = yaml.safe_load_all(r.text)
    crds[crd] = [crd for crd in all_crds]

for name, sub_crds in crds.items():
    for crd in sub_crds:
        file_name = (
            crd["metadata"]["name"].removesuffix(".cilium.io").removeprefix("cilium")
        )
        rust_code = ""
        # Save the CRD as a tmp yaml file
        with tempfile.NamedTemporaryFile(mode="w") as f:
            yaml.dump(crd, f)
            tmp_file = f.name
            rust_code = subprocess.run(
                [
                    "kopium",
                    "-f",
                    tmp_file,
                    "--schema=derived",
                    "--docs",
                    "-b",
                    "--derive=Default",
                    "--derive=PartialEq",
                ],
                capture_output=True,
            )
            if rust_code.returncode != 0:
                print(rust_code.stderr.decode("utf-8"))
                exit(1)
            rust_code = rust_code.stdout.decode("utf-8")

        rust_code = rust_code.replace(
            f"// kopium command: kopium -f {tmp_file} --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision",
            f"// kopium command: kopium -f {file_name}.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision",
        )
        rust_code = "\n".join(
            [
                line.replace("#[builder(", '#[cfg_attr(feature = "builder", builder(')
                .strip()
                .removesuffix("]")
                + ")]"
                if "#[builder(" in line
                else line
                for line in rust_code.split("\n")
            ]
        )
        rust_code = rust_code.replace(
            # TODO: Get a better type for this
            "CiliumClusterwideEnvoyConfigResources",
            "serde_json::Value",
        ).replace(
            # TODO: Get a better type for this
            "CiliumEnvoyConfigResources",
            "serde_json::Value",
        )
        rust_code = "\n".join(
            [
                line.replace(
                    ", TypedBuilder, Default, PartialEq, JsonSchema)]",
                    ', Default, PartialEq)]\n#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]\n#[cfg_attr(not(feature = "schemars"), kube(schema="disabled"))]',
                ).replace(
                    ", TypedBuilder, PartialEq, JsonSchema)]",
                    ', PartialEq)]\n#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]\n#[cfg_attr(not(feature = "schemars"), kube(schema="disabled"))]',
                ).replace(
                    ", Default, PartialEq, JsonSchema)]",
                    ', Default, PartialEq)]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]\n#[cfg_attr(not(feature = "schemars"), kube(schema="disabled"))]',
                ).replace(
                    ", PartialEq, JsonSchema)]",
                    ', PartialEq)]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]\n#[cfg_attr(not(feature = "schemars"), kube(schema="disabled"))]',
                )
                if line.startswith("#[derive(") and "CustomResource" in line
                else line.replace(
                    ", TypedBuilder, Default, PartialEq, JsonSchema)]",
                    ', Default, PartialEq)]\n#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]',
                ).replace(
                    ", TypedBuilder, PartialEq, JsonSchema)]",
                    ', PartialEq)]\n#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]',
                ).replace(
                    ", Default, PartialEq, JsonSchema)]",
                    ', Default, PartialEq)]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]',
                ).replace(
                    ", PartialEq, JsonSchema)]",
                    ', PartialEq)]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]',
                )
                if line.startswith("#[derive(")
                else line
                for line in rust_code.split("\n")
            ]
        )
        rust_code = (
            rust_code.replace(
                "pub use typed_builder::TypedBuilder;",
                '#[cfg(feature = "builder")]\npub use typed_builder::TypedBuilder;',
            )
            .replace(
                "pub use schemars::JsonSchema;",
                '#[cfg(feature = "schemars")]\npub use schemars::JsonSchema;',
            )
            .replace(
                "pub use kube::CustomResource;", "pub use kube_derive::CustomResource;"
            )
            .replace(
                '#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]\npub enum',
                '#[cfg_attr(feature = "schemars", derive(JsonSchema))]\npub enum',
            )
        )
        rust_file = f"./src/{file_name}.rs"
        with open(rust_file, "w") as f:
            f.write(rust_code)
        # Format the code
        subprocess.run(["rustfmt", rust_file])
        if name in crds_stable:
            rust_lib += f"pub mod {file_name};\npub use {file_name}::*;\n"
        else:
            rust_lib += f'#[cfg(feature = "alpha")]\npub mod {file_name};\n#[cfg(feature = "alpha")]\npub use {file_name}::*;\n'

with open("./src/lib.rs", "w") as f:
    f.write(rust_lib)
