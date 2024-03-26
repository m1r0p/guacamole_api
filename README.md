[![License](https://img.shields.io/badge/license-MIT-green)](https://choosealicense.com/licenses/mit/)
[![Version](https://img.shields.io/badge/version-0.1.0-blue)]()

# This script can interact with Apache Guacamole REST API

## Installation

Install Rust by [rustup](https://rustup.rs/)

## Usage 

```cargo run -- --config <path_to_your_config>``` 

or 

```<path_to_compiled_bin_file> --config <path_to_your_config>```

### config example

```
{
    "gua_proto_address":"http://<your_ip_or_hostname>:8080/guacamole",
    "gua_user":"<guacamole_admin_user>",
    "gua_pass":"<password_for_admin_user>",
    "rdp_hosts_file":"<path_to_csv_with_rdp_hosts>",
    "vnc_hosts_file":"<path_to_csv_with_vnc_hosts>",
    "manual_stands_assign_group":"AD_group_for_manual_stands",
    "automation_stands_assign_group":"AD_group_for_automation_stands",
    "gua_broadcast_map":"{'<network1>':'<broadcast_address1>', '<network2>':'<broadcast_address2>'}"
}



```

## Links

- [REST API reference](https://github.com/ridvanaltun/guacamole-rest-api-documentation/tree/master/docs)
- [curl examples](https://gist.github.com/atomlab/376901845c3d474d1e60e6b7a3affaae)


## License

[MIT](https://choosealicense.com/licenses/mit/)
