mod conf;
mod enums;
mod functions;
mod structures;

///// functions attaching
use crate::functions::assign_gua_user_to_conn::*;
//use crate::functions::create_gua_conn_group::*;
use crate::functions::create_gua_rdp_connection::*;
use crate::functions::create_gua_token::*;
use crate::functions::create_gua_vnc_connection::*;
use crate::functions::delete_gua_connection::*;
//use crate::functions::delete_gua_conn_group::*;
use crate::functions::delete_gua_token::*;
use crate::functions::get_config_params::*;
//use crate::functions::get_gua_conn_groups::*;
use crate::functions::assign_conn_to_user_group::*;
use crate::functions::get_broadcast_map::*;
use crate::functions::get_gua_connections::*;
use crate::functions::parse_csv::*;
use crate::functions::update_gua_rdp_connection::*;
use crate::functions::update_gua_vnc_connection::*;

use ipnet::Ipv4Net;
use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config_path: String = String::new();
    //let mut input_csv_path: String = String::new();
    let mut i: usize = 0;
    for word in args.iter() {
        if word.as_str().eq("--config") {
            config_path.push_str(args[i + 1].as_str());
        }
        i = i + 1;
    }

    let vec_config: Vec<String> = get_config_params(config_path).unwrap();

    let broadcast_map: HashMap<Ipv4Net, String> = get_broadcast_map(vec_config[7].clone());

    //// create token for this session
    let token: String = create_gua_token(&vec_config[0], &vec_config[1], &vec_config[2]).unwrap();
    println!("token: {}", &token);

    //// get existent guacamole connection groups
    //let conn_grp_list: Vec<GuaConnGrp> = get_gua_conn_groups(&vec_config[0], &token).unwrap();

    //// get existent guacamole connections
    let connections: Vec<GuaConn> = get_gua_connections(&vec_config[0], &token).unwrap();

    //// parse .csv and get actual RDP host list
    let rdp_hosts: Vec<Host> = parse_csv(&vec_config[3]).unwrap();

    ////create separate vector of RDP hostnames for comparing
    let mut rdp_host_names: Vec<String> = Vec::new();
    for host in rdp_hosts.iter() {
        rdp_host_names.push(host.hostname.clone());
    }

    //// parse .csv and get actual VNC host list
    let vnc_hosts: Vec<Host> = parse_csv(&vec_config[4]).unwrap();

    ////create separate vector of VNC hostnames for comparing
    let mut vnc_host_names: Vec<String> = Vec::new();
    for host in vnc_hosts.iter() {
        vnc_host_names.push(host.hostname.clone());
    }

    ////create separate vector for connection group names
    //let mut conn_grp_names: Vec<String> = Vec::new();
    //for grp_name in conn_grp_list.iter() {
    //    conn_grp_names.push(grp_name.name.clone());
    //}

    //// create connection groups
    //for host_name in rdp_host_names.iter() {
    //    if !conn_grp_names.contains(&host_name) {
    //        println!("CREATING COONECTION GROUP - {}", &host_name);
    //        _ = create_gua_conn_group(&vec_config[1], &token, &host_name);
    //    } else {
    //        println!("COONECTION GROUP {} EXIST. SKIPING", &host_name);
    //        continue;
    //    }
    //}

    //// get existent guacamole connection groups again
    //let conn_grp_list: Vec<GuaConnGrp> = get_gua_conn_groups(&vec_config[0], &token).unwrap();

    //// create static parent connection group ROOT for backward compability
    let conn_group_identifier: String = String::from("ROOT");

    //// compare attributes and update or delete existent RDP connections
    if connections.len() > 0 {
        for conn in connections.iter() {
            //println!("{}", &i.name);
            //let conn_det: [String; 5] =
            //    get_gua_connection_details(&vec_config[1], &token, &i.identifier).unwrap();
            //println!("{:?}", conn_det);
            if conn.protocol == "rdp" {
                if !rdp_host_names.contains(&conn.name) {
                    //println!("DELETING CONNECTION");
                    //println!("{}", &conn.name);
                    //_ = delete_gua_connection(&vec_config[0], &token, &conn.identifier);
                    continue;
                } else {
                    println!("UPDATING EXISTENT RDP CONNECTION");
                    for rdp_host in rdp_hosts.iter() {
                        if rdp_host.hostname == conn.name {
                            //for conn_grp in conn_grp_list.iter() {
                            //    if conn_grp.name == rdp_host.hostname {
                            //        println!("{} - {}", &conn.name, &conn.identifier);
                            //        _ = update_gua_rdp_connection(
                            //            &vec_config[0],
                            //            &token,
                            //            &rdp_host,
                            //            &conn.identifier,
                            //            &conn_grp.identifier,
                            //        );
                            //    }
                            //}
                            let mut brd_address: String = String::new();
                            let host_ip: Ipv4Addr = rdp_host.ipv4.parse().unwrap();
                            for (b_net, b_addr) in broadcast_map.iter() {
                                if b_net.contains(&host_ip) {
                                    brd_address = b_addr.to_string().replace("\"", "");
                                }
                            }
                            println!("{} - {} BRD_ADDR - {}", &conn.name, &conn.identifier, &brd_address);
                            _ = update_gua_rdp_connection(
                                &vec_config[0],
                                &token,
                                &rdp_host,
                                &conn.identifier,
                                &conn_group_identifier,
                                &brd_address,
                            );
                        }
                    }
                }
            }

            if conn.protocol == "vnc" {
                if !vnc_host_names.contains(&conn.name) {
                    println!("DELETING CONNECTION");
                    println!("{}", &conn.name);
                    _ = delete_gua_connection(&vec_config[0], &token, &conn.identifier);
                    //continue;
                } else {
                    println!("UPDATING EXISTENT VNC CONNECTION");
                    for vnc_host in vnc_hosts.iter() {
                        if vnc_host.hostname == conn.name {
                            //for conn_grp in conn_grp_list.iter() {
                            //    if conn_grp.name == vnc_host.hostname {
                            //        println!("{} - {}", &conn.name, &conn.identifier);
                            //        _ = update_gua_vnc_connection(
                            //            &vec_config[0],
                            //            &token,
                            //            &vnc_host,
                            //            &conn.identifier,
                            //            &conn_grp.identifier,
                            //        );
                            //    }
                            //}
                            let mut brd_address: String = String::new();
                            let host_ip: Ipv4Addr = vnc_host.ipv4.parse().unwrap();
                            for (b_net, b_addr) in broadcast_map.iter() {
                                if b_net.contains(&host_ip) {
                                    brd_address = b_addr.to_string().replace("\"", "");
                                }
                            }
                            println!("{} - {} BRD_ADDR - {}", &conn.name, &conn.identifier, &brd_address);
                            _ = update_gua_vnc_connection(
                                &vec_config[0],
                                &token,
                                &vnc_host,
                                &conn.identifier,
                                &conn_group_identifier,
                                &brd_address,
                            );
                        }
                    }
                }
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

    //// create non existent RDP connections
    for rdp_host in rdp_hosts.iter() {
        if !connection_names.contains(&rdp_host.hostname) {
            println!("CREATING RDP CONNECTION");
            println!("{}", &rdp_host.hostname);
            //for conn_grp in conn_grp_list.iter() {
            //    if conn_grp.name == rdp_host.hostname {
            //        _ = create_gua_rdp_connection(
            //            &vec_config[0],
            //            &token,
            //            &rdp_host,
            //            &conn_grp.identifier,
            //        );
            //    }
            //}
            let mut brd_address: String = String::new();
            let host_ip: Ipv4Addr = rdp_host.ipv4.parse().unwrap();
            for (b_net, b_addr) in broadcast_map.iter() {
                if b_net.contains(&host_ip) {
                    brd_address = b_addr.to_string().replace("\"", "");
                }
            }
            _ = create_gua_rdp_connection(
                &vec_config[0],
                &token,
                &rdp_host,
                &conn_group_identifier,
                &brd_address,
            );
        }
    }

    //// create non existent VNC connections
    for vnc_host in vnc_hosts.iter() {
        if !connection_names.contains(&vnc_host.hostname) {
            println!("CREATING VNC CONNECTION");
            println!("{}", &vnc_host.hostname);
            //for conn_grp in conn_grp_list.iter() {
            //    if conn_grp.name == vnc_host.hostname {
            //        _ = create_gua_vnc_connection(
            //            &vec_config[0],
            //            &token,
            //            &vnc_host,
            //            &conn_grp.identifier,
            //        );
            //    }
            //}
            let mut brd_address: String = String::new();
            let host_ip: Ipv4Addr = vnc_host.ipv4.parse().unwrap();
            for (b_net, b_addr) in broadcast_map.iter() {
                if b_net.contains(&host_ip) {
                    brd_address = b_addr.to_string().replace("\"", "");
                }
            }
            _ = create_gua_vnc_connection(
                &vec_config[0],
                &token,
                &vnc_host,
                &conn_group_identifier,
                &brd_address,
            );
        }
    }

    //// get existent guacamole connections again
    let connections: Vec<GuaConn> = get_gua_connections(&vec_config[0], &token).unwrap();

    for conn in connections.iter() {
        match &conn.proto_based_attributes {
            ProtoBasedAttributes::RDP(x) => {
                if x.username.as_str() != "None" {
                    println!("ASSIGN {} to {}", &conn.name, &x.username);
                    _ = assign_gua_user_to_conn(&vec_config[0], &token, &conn);
                } else {
                    println!("SKIPING ASSIGN FOR USER {}", &x.username);
                    continue;
                }
                //println!("{:?}", x.username);
            }

            ProtoBasedAttributes::VNC(_x) => {
                if conn.name.starts_with("stand") {
                    println!("ASSIGN {} to GROUP {}", &conn.name, &vec_config[5]);
                    _ = assign_conn_to_user_group(&vec_config[0], &token, &conn, &vec_config[5]);
                }
                if conn.name.starts_with("autostand") {
                    println!("ASSIGN {} to GROUP {}", &conn.name, &vec_config[6]);
                    _ = assign_conn_to_user_group(&vec_config[0], &token, &conn, &vec_config[6]);
                }
                //_ => continue,
            }
        }
        //if conn.username.as_str() != "None" {
        //    _ = assign_gua_user_to_conn(&vec_config[1], &token, &conn);
        //} else {
        //    continue;
        //}

        //match &conn.proto_based_attributes {
        //    ProtoBasedAttributes::RDP(x) => println!("{:?}", x.username),
        //    _ => continue,
        //}

        //println!("{:?}", &conn.proto_based_attributes::RDP.username);
    }

    //// REMOVE ALL CONNECTION GROUPS
    //let conn_grp_list: Vec<GuaConnGrp> = get_gua_conn_groups(&vec_config[0], &token).unwrap();
    //for conn_grp in conn_grp_list.iter() {
    //    _ = delete_gua_conn_group(&vec_config[0], &token, &conn_grp.identifier);
    //}

    //// deleting token for this session (cleaning)
    _ = delete_gua_token(&vec_config[0], &token);
}
