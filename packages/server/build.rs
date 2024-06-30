use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/helloworld.proto")?;
    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // println!("{:?}", out_dir);
    // tonic_build::configure()
    //     .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
    //     .compile(&["proto/helloworld.proto"], &["/proto"])
    //     .unwrap();
    // Ok(())
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile(&["proto/helloworld.proto"], &["proto"])
        .expect("Failed to compile protos");
    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // println!("out_dir: {:?}", out_dir);
    // let descriptor_path = out_dir.join("helloworld_descriptor.bin");
    // tonic_build::configure()
    //     .file_descriptor_set_path(&descriptor_path)
    //     // .format(true)
    //     .compile(&["proto/helloworld.proto"], &["proto/"])?;
    Ok(())
}
