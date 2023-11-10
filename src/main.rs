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

    //// create token for this session
    let token: String = create_gua_token(&vec_config[1], &vec_config[2], &vec_config[3]).unwrap();
    println!("token: {}", &token);

    //// get existent guacamole connections
    let connections: Vec<GuaConn> = get_gua_connections(&vec_config[1], &token).unwrap();

    //// parse .csv and get actual host list from SCCM
    let sccm_hosts: Vec<SccmHost> = parse_csv(&vec_config[0]).unwrap();

    ////create separate vector of PC names for comparing hostnames
    let mut sccm_host_names: Vec<String> = Vec::new();
    for host in sccm_hosts.iter() {
        sccm_host_names.push(host.hostname.clone());
    }

    //// compare attributes and update or delete existent RDP connections
    if connections.len() > 0 {
        for i in connections.iter() {
            //println!("{}", &i.name);
            //let conn_det: [String; 5] =
            //    get_gua_connection_details(&vec_config[1], &token, &i.identifier).unwrap();
            //println!("{:?}", conn_det);
            if i.protocol == "rdp" {
                if !sccm_host_names.contains(&i.name) {
                    println!("DELETING CONNECTION");
                    println!("{}", &i.name);
                    //_ = delete_gua_connection(&vec_config[1], &token, &i.identifier);
                } else {
                    println!("UPDATING EXISTENT CONNECTION");
                    for j in sccm_hosts.iter() {
                        if j.hostname == i.name {
                            println!("{} - {}", &i.name, &i.identifier);
                            _ = update_gua_connection(&vec_config[1], &token, &j, &i.identifier);
                        }
                    }
                }
                //println!("{:?}", &i);
            }
        }
    }

    ////create separate vector of connection names for comparing hostnames
    let mut connection_names: Vec<String> = Vec::new();
    for conn in connections.iter() {
        connection_names.push(conn.name.clone());
    }

    //// create non existent connections
    for i in sccm_hosts.iter() {
        if !connection_names.contains(&i.hostname) {
            println!("CREATING CONNECTION");
            println!("{}", &i.hostname);
            //_ = create_gua_connection(&vec_config[1], &token, &i);
        }
    }

    // deleting token for this session (cleaning)
    _ = delete_gua_token(&vec_config[1], &token);
}
