use std::{path::PathBuf, io::Write};

use embuild::{
    self, bingen,
    build::{CfgArgs, LinkArgs},
    cargo, symgen,
};
fn main() -> anyhow::Result<()> {
    build_config()?;
    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
    LinkArgs::output_propagated("ESP_IDF")?;

    let cfg = CfgArgs::try_from_env("ESP_IDF")?;

    if cfg.get("esp32s2").is_some() {
        // Future; might be possible once https://github.com/rust-lang/cargo/issues/9096 hits Cargo nightly:
        //let ulp_elf = PathBuf::from(env::var_os("CARGO_BIN_FILE_RUST_ESP32_ULP_BLINK_rust_esp32_ulp_blink").unwrap());

        let ulp_elf = PathBuf::from("ulp").join("rust-esp32-ulp-blink");
        cargo::track_file(&ulp_elf);

        // This is where the RTC Slow Mem is mapped within the ESP32-S2 memory space
        let ulp_bin = symgen::Symgen::new(&ulp_elf, 0x5000_0000_u64).run()?;
        cargo::track_file(ulp_bin);

        let ulp_sym = bingen::Bingen::new(ulp_elf).run()?;
        cargo::track_file(ulp_sym);
    }

    cfg.output();

    Ok(())
}
include!("./src/config/structs.rs");
fn build_config() -> anyhow::Result<()> {
    build_app_config()?;
    build_device_config()?;
    Ok(())
}
struct MyRandom;
impl rand::RngCore for MyRandom {
    fn next_u32(&mut self) -> u32 {
        rand::thread_rng().next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        rand::thread_rng().next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand::thread_rng().fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
       rand::thread_rng().try_fill_bytes(dest)
    }
}
impl rand::CryptoRng for MyRandom {

}
impl nrq_engine::CoreCryptoRng for MyRandom {

}
fn build_device_config() -> anyhow::Result<()> {
    let device_data:nrq_engine::protocol::device::Device = match std::fs::read("./device.json").ok().and_then(|d|serde_json::from_slice(&d).ok()) {
        Some(d) => {d},
        None => {
            nrq_engine::init_random_provider(Box::new(MyRandom));
            let rand_device = nrq_engine::protocol::device::Device::random();
            std::fs::write("./device.json", serde_json::to_string_pretty(&rand_device)?)?;
            rand_device
        },
    };
    uneval::to_file(device_data, "./out/device.out")?;
    Ok(())
}

fn build_app_config() -> anyhow::Result<()> {
    let app_data:std::io::Result<Vec<u8>> = std::fs::read("./app.json");
    let app:AppConfig = match app_data {
        Ok(ref config_vec) => {
            serde_json::from_slice(config_vec)?
        },
        Err(_) => {
            let default_config = AppConfig::default();
            
            std::fs::write("./app.json", serde_json::to_string_pretty(&default_config)?.as_bytes())?;
            default_config
        }
    };
    let mut file = std::fs::File::create("./out/app.config.out")?;
    let  out_string = uneval::to_string(app)?;
    let out_string = out_string.replace(".into()", "");
    file.write_fmt(format_args!("{}",out_string))?;
    // uneval::to_file(app, "./src/config/app.config.out")?;
    Ok(())
}