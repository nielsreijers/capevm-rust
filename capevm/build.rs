extern crate toml;

use std::fs;
use std::path::Path;
use toml::Value;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=vm-config.toml");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("enabled_plugins.rs");

    let contents: String = fs::read_to_string("vm-config.toml").unwrap();
    let cargo_toml = contents.parse::<Value>().unwrap();

    let vm_plugins =
        if let Some(capevm_plugins) = cargo_toml.get("capevm")
                                        .and_then(Value::as_table)
                                        .and_then(|table| table.get("plugins"))
                                        .and_then(Value::as_array) {
            capevm_plugins.iter().filter_map(|v| v.as_str()).collect::<Vec<&str>>()
        } else {
            Vec::<&str>::default()
        };

    let mod_imports =
        vm_plugins.iter()
            .map(|name| format!(r#"
                #[path = "{manifest_dir}/src/plugins/{name}/mod.rs"]
                mod {name};"#, manifest_dir=manifest_dir, name=name))
            .collect::<Vec<_>>().join("\n");
    let mod_inits =
        vm_plugins.iter()
            .map(|name| format!("
                {}::init();", name))
            .collect::<Vec<_>>().join("\n");

    let generated_code =
        format!("{}
            
            pub fn init() {{
                {}
            }}", mod_imports, mod_inits);

    fs::write(dest_path, generated_code.as_bytes()).unwrap();
}
