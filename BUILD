load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@bazel_skylib//rules:common_settings.bzl", "bool_flag")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load("@rules_rust//wasm_bindgen:defs.bzl", "rust_wasm_bindgen")

package(default_visibility = ["//visibility:public"])

config_setting(
    name = "fastbuild",
    values = {
        "compilation_mode": "fastbuild",
    },
)

config_setting(
    name = "debug",
    values = {
        "compilation_mode": "dbg",
    },
)

rust_binary(
    name = "app",
    srcs = ["src/bin/app.rs"],
    aliases = aliases(),
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    rustc_flags = select({
        ":debug": [
            "-Copt-level=0",
        ],
        ":fastbuild": [],
        "//conditions:default": [
            "-Clto",
            "-Ccodegen-units=1",
            "-Cpanic=abort",
            "-Copt-level=z",
        ],
    }),
    deps = all_crate_deps(
        normal = True,
    ) + [
        ":todomvc_wasm",
        "@rules_rust//wasm_bindgen/3rdparty:wasm_bindgen",
    ],
)

rust_library(
    name = "todomvc_wasm",
    srcs = glob(
        include = [
            "src/**/*.rs",
        ],
        exclude = ["src/bin/**"],
    ),
    aliases = aliases(),
    edition = "2021",
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    deps = all_crate_deps(
        normal = True,
    ),
)

rust_wasm_bindgen(
    name = "app_wasm",
    target = "web",
    wasm_file = ":app",
)

