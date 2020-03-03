#[macro_use]
extern crate cfg_if;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let _ = env_logger::init();

    // The source directory.
    let src = PathBuf::from("../tests/src");
    let includes = &[src.clone()];

    let mut config = prost_build::Config::new();

    // Force btree_map everywhere for no_std compat
    config.btree_map(&["."]);

    // Tests for custom attributes
    config.type_attribute("Foo.Bar_Baz.Foo_barBaz", "#[derive(Eq, PartialOrd, Ord)]");
    config.type_attribute(
        "Foo.Bar_Baz.Foo_barBaz.fuzz_buster",
        "#[derive(Eq, PartialOrd, Ord)]",
    );
    config.type_attribute("Foo.Custom.Attrs.Msg", "#[allow(missing_docs)]");
    config.type_attribute("Foo.Custom.Attrs.Msg.field", "/// Oneof docs");
    config.type_attribute("Foo.Custom.Attrs.AnEnum", "#[allow(missing_docs)]");
    config.type_attribute("Foo.Custom.Attrs.AnotherEnum", "/// Oneof docs");
    config.type_attribute(
        "Foo.Custom.OneOfAttrs.Msg.field",
        "#[derive(Eq, PartialOrd, Ord)]",
    );
    config.field_attribute("Foo.Custom.Attrs.AnotherEnum.C", "/// The C docs");
    config.field_attribute("Foo.Custom.Attrs.AnotherEnum.D", "/// The D docs");
    config.field_attribute("Foo.Custom.Attrs.Msg.field.a", "/// Oneof A docs");
    config.field_attribute("Foo.Custom.Attrs.Msg.field.b", "/// Oneof B docs");

    config
        .compile_protos(&[src.join("ident_conversion.proto")], includes)
        .unwrap();

    config
        .compile_protos(&[src.join("nesting.proto")], includes)
        .unwrap();

    config
        .compile_protos(&[src.join("recursive_oneof.proto")], includes)
        .unwrap();

    config
        .compile_protos(&[src.join("custom_attributes.proto")], includes)
        .unwrap();

    config
        .compile_protos(&[src.join("oneof_attributes.proto")], includes)
        .unwrap();

    config
        .compile_protos(&[src.join("no_unused_results.proto")], includes)
        .unwrap();

    config
        .compile_protos(&[src.join("default_enum_value.proto")], includes)
        .unwrap();

    config
        .compile_protos(&[src.join("groups.proto")], includes)
        .unwrap();

    config
        .compile_protos(&[src.join("well_known_types.proto")], includes)
        .unwrap();

    config
        .compile_protos(
            &[src.join("packages/widget_factory.proto")],
            &[src.join("packages")],
        )
        .unwrap();

    let out_dir =
        &PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
            .join("extern_paths");
    fs::create_dir_all(out_dir).expect("failed to create prefix directory");
    config.out_dir(out_dir);

    // Compile some of the module examples as an extern path. The extern path syntax is edition
    // specific, since the way crate-internal fully qualified paths has changed.
    cfg_if! {
        if #[cfg(feature = "edition-2015")] {
            const EXTERN_PATH: &str = "::packages::gizmo";
        } else {
            const EXTERN_PATH: &str = "crate::packages::gizmo";
        }
    };
    config.extern_path(".packages.gizmo", EXTERN_PATH);

    config
        .compile_protos(
            &[src.join("packages").join("widget_factory.proto")],
            &[src.join("packages")],
        )
        .unwrap();
}
