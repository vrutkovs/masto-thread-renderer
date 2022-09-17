use npm_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=public/");

    let npm_path = std::env::current_dir().unwrap().join("public");

    let npm_status = NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile()?)
        .set_path(npm_path)
        .init_env()
        .install(None)
        .run("css")
        .exec()?;

    if !npm_status.success() {
        println!("cargo:warning=npm failed with: {}", npm_status);
    }

    Ok(())
}
