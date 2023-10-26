mod functions;
use functions::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config_path: String = String::new();
    let mut input_csv_path: String = String::new();
    let mut i: usize = 0;
    for word in args.iter() {
        if word.as_str().eq("--config") {
            config_path.push_str(args[i + 1].as_str());
        }
        if word.as_str().eq("--input_csv") {
            input_csv_path.push_str(args[i + 1].as_str());
        }

        i = i + 1;
    }

    let vec_config: Vec<String> = get_config_params(config_path).unwrap();

    let token: String = create_gua_token(&vec_config[1], &vec_config[2], &vec_config[3]).unwrap();
    //println!("{}", &token);
    let connections: Vec<GuaConn> = get_gua_connections(&vec_config[1], &token).unwrap();
    if connections.len() > 0 {
        println!("################### START DELETING CONNECTIONS ####################");
        for i in connections.iter() {
            println!("{:?}", i.identifier);
            _ = delete_gua_connection(&vec_config[1], &token, &i.identifier);
            println!("{} - deleted", &i.name);
        }
    }

    let sccm_hosts: Vec<SccmHost> = parse_csv(&vec_config[0]).unwrap();
    println!("################### START CREATING CONNECTIONS ####################");
    for i in sccm_hosts {
        _ = create_gua_connection(&vec_config[1], &token, &i);
        println!("{} - created", &i.hostname);
    }

    _ = delete_gua_token(&vec_config[1], &token);

}
