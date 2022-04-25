use std::io::Result;
use std::path::Path;

fn recursion<P: AsRef<Path>>(v: &mut Vec<String>, dir: P) -> Result<()> {
    let rd = std::fs::read_dir(dir)?;
    for x in rd {
        let de = x?;
        let path = de.path();
        if path.is_dir() {
            recursion(v, path)?;
        } else {
            let path = path.into_os_string().into_string().unwrap();
            if path.ends_with(".proto") {
                v.push(path);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut v = Vec::<String>::new();
    recursion(&mut v, "src/pb")?;
    let mut config = prost_build::Config::new();
    config
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir("src/pb")
        .compile_protos(&v, &["src/pb"])
        .ok();
    Ok(())
}
