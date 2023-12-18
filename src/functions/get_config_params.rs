use config::{Config, File, FileFormat};
use std::error::Error;

pub fn get_config_params(string_path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut config_params: Vec<String> = Vec::new();

    let mut builder = Config::builder();
    builder = builder.set_default("default", "1")?;
    builder = builder.add_source(File::new(&string_path, FileFormat::Json));
    builder = builder.set_override("override", "1")?;
    let raw_conf = builder.build().unwrap();
    config_params.push(raw_conf.get("gua_proto_address").unwrap());
    config_params.push(raw_conf.get("gua_user").unwrap());
    config_params.push(raw_conf.get("gua_pass").unwrap());
    config_params.push(raw_conf.get("rdp_hosts_file").unwrap());
    config_params.push(raw_conf.get("vnc_hosts_file").unwrap());
    config_params.push(raw_conf.get("manual_stands_assign_group").unwrap());
    config_params.push(raw_conf.get("automation_stands_assign_group").unwrap());


    return Ok(config_params);
}
