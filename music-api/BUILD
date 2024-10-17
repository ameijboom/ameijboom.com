load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_tarball")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "music-api",
    srcs = glob(["src/**/*.rs"]),
    edition = "2021",
    deps = all_crate_deps(normal = True),
)

pkg_tar(
    name = "layer",
    srcs = [":music-api"],
)

oci_image(
    name = "image",
    base = "@distroless_cc",
    entrypoint = ["/music-api"],
    tars = [":layer"],
)

oci_tarball(
    name = "image.tar",
    image = ":image",
    repo_tags = ["music-api:latest"],
)
