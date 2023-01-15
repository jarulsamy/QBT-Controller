use embed_manifest::manifest::ActiveCodePage::System as SystemCodePage;
use embed_manifest::manifest::ExecutionLevel::AsInvoker;
use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    // Ensure the prper manifest is included
    let manifest = new_manifest("QBT Controller Manifest")
        .active_code_page(SystemCodePage)
        .requested_execution_level(AsInvoker);
    embed_manifest(manifest).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
