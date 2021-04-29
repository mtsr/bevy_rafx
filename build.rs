use std::error::Error;

use rafx_shader_processor::{self, ShaderProcessorArgs};

// Example custom build script.
fn main() -> Result<(), Box<dyn Error>> {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=raw-shaders/*.*");
    println!("cargo:rerun-if-changed=build.rs");

    env_logger::Builder::from_default_env().init();

    let args = ShaderProcessorArgs {
        glsl_file: None,
        spv_file: None,
        rs_file: Some("src/shaders.rs".into()),
        metal_generated_src_file: None,
        cooked_shader_file: None,
        glsl_files: Some(vec!["raw-shaders/*".into()]),
        spv_path: Some("assets/processed-shaders".into()),
        rs_path: None,
        metal_generated_src_path: Some("assets/processed-shaders".into()),
        cooked_shaders_path: Some("assets/cooked-shaders".into()),
        shader_kind: None,
        trace: true,
        optimize_shaders: false,
    };

    rafx_shader_processor::run(&args)?;

    Ok(())
}
