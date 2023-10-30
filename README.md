# This scrtipt can interact with Apache Guacamole REST API

## Installation

Install Rust by [rustup](https://rustup.rs/)

## Usage 

```cargo run -- --config <path_to_your_config>``` 

or 

```<path_to_compiled_bin_file> --config <path_to_your_config>```

### config example

```
{
    "csv_input_file":"<path_to_hostlist.csv>",
    "gua_proto_address":"http://<your_ip_or_hostname>:8080/guacamole",
    "gua_user":"guacadmin",
    "gua_pass":"guacadmin"
}

```

## Links

- [REST API reference](https://github.com/ridvanaltun/guacamole-rest-api-documentation/tree/master/docs)
- [curl examples](https://gist.github.com/atomlab/376901845c3d474d1e60e6b7a3affaae)


## License

[MIT](https://choosealicense.com/licenses/mit/)
