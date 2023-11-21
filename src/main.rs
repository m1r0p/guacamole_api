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

    //// get existent guacamole connection groups
    let conn_grp_list: Vec<GuaConnGrp> = get_gua_conn_groups(&vec_config[1], &token).unwrap();

    //// get existent guacamole connections
    let connections: Vec<GuaConn> = get_gua_connections(&vec_config[1], &token).unwrap();

    //// parse .csv and get actual host list from SCCM
    let sccm_hosts: Vec<SccmHost> = parse_csv(&vec_config[0]).unwrap();

    ////create separate vector of PC names for comparing hostnames
    let mut sccm_host_names: Vec<String> = Vec::new();
    for host in sccm_hosts.iter() {
        sccm_host_names.push(host.hostname.clone());
    }

    ////create separate vector for connection group names
    let mut conn_grp_names: Vec<String> = Vec::new();
    for grp_name in conn_grp_list.iter() {
        conn_grp_names.push(grp_name.name.clone());
    }

    for host_name in sccm_host_names.iter() {
        if !conn_grp_names.contains(&host_name) {
            println!("CREATING COONECTION GROUP - {}", &host_name);
            _ = create_gua_conn_group(&vec_config[1], &token, &host_name);
        } else {
            println!("COONECTION GROUP {} EXIST. SKIPING", &host_name);
            continue;
        }
    }

    //// get existent guacamole connection groups again
    let conn_grp_list: Vec<GuaConnGrp> = get_gua_conn_groups(&vec_config[1], &token).unwrap();

    //// compare attributes and update or delete existent RDP connections
    if connections.len() > 0 {
        for conn in connections.iter() {
            //println!("{}", &i.name);
            //let conn_det: [String; 5] =
            //    get_gua_connection_details(&vec_config[1], &token, &i.identifier).unwrap();
            //println!("{:?}", conn_det);
            if conn.protocol == "rdp" {
                if !sccm_host_names.contains(&conn.name) {
                    //println!("DELETING CONNECTION");
                    //println!("{}", &i.name);
                    //_ = delete_gua_connection(&vec_config[1], &token, &i.identifier);
                    continue;
                } else {
                    println!("UPDATING EXISTENT CONNECTION");
                    for sccm_host in sccm_hosts.iter() {
                        if sccm_host.hostname == conn.name {
                            for conn_grp in conn_grp_list.iter() {
                                if conn_grp.name == sccm_host.hostname {
                                    println!("{} - {}", &conn.name, &conn.identifier);
                                    _ = update_gua_connection(
                                        &vec_config[1],
                                        &token,
                                        &sccm_host,
                                        &conn.identifier,
                                        &conn_grp.identifier,
                                    );
                                }
                            }
                        }
                    }
                }
                //println!("{:?}", &i);
            }
        }
    } else {
        println!("No one connection found, nothing to update");
    }

    ////create separate vector of connection names for comparing hostnames
    let mut connection_names: Vec<String> = Vec::new();
    for conn in connections.iter() {
        connection_names.push(conn.name.clone());
    }

    //// create non existent connections
    for sccm_host in sccm_hosts.iter() {
        if !connection_names.contains(&sccm_host.hostname) {
            println!("CREATING CONNECTION");
            println!("{}", &sccm_host.hostname);
            for conn_grp in conn_grp_list.iter() {
                if conn_grp.name == sccm_host.hostname {
                    _ = create_gua_connection(
                        &vec_config[1],
                        &token,
                        &sccm_host,
                        &conn_grp.identifier,
                    );
                }
            }
        }
    }

    //// get existent guacamole connections again
    let connections: Vec<GuaConn> = get_gua_connections(&vec_config[1], &token).unwrap();

    for conn in connections.iter() {
        //match
        if conn.username.as_str() != "None" {
            _ = assign_gua_user_to_conn(&vec_config[1], &token, &conn);
        } else {
            continue;
        }

        //match &conn.proto_based_attributes {
        //    ProtoBasedAttributes::RDP(x) => println!("{:?}", x.username),
        //    _ => continue,
        //}

        //println!("{:?}", &conn.proto_based_attributes::RDP.username);
    }

    // deleting token for this session (cleaning)
    _ = delete_gua_token(&vec_config[1], &token);
}
