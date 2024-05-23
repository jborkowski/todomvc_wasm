load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")


rust_binary(
    name = "server",
    srcs = ["src/main.rs"],
    aliases = aliases(),
    data = [
        "//:app_wasm",
    ],
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    rustc_flags = select({
        "//:debug": [
            "-Copt-level=0",
        ],
        "//:fastbuild": [],
        "//conditions:default": [],
    }),
    deps = all_crate_deps(
        normal = True,
    ) + [
        "//:todomvc_wasm",
    ],
)
