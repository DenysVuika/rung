# rung: (Ru)st + A(ng)ular

Various tools for Angular projects written in Rust.

Commands:

- Serve
- Check JSON
- Check Header

## Help

Use the `--help` argument to get more details about the program or specific command:

```shell
rung --help
rung <command> --help
```

## Serve

Runs a lightweight web server.

Usage:

```shell
rung serve [OPTIONS] <dir>

OPTIONS:
    -h, --host <HOST>    Host address [default: 127.0.0.1]
    -p, --port <PORT>    Port number [default: 8080]
```

You can get more details by running the following command: 

```shell
rung serve --help
```

Examples:

```shell
# serves application at http://localhost:8081
rung serve ./dist/app1 -p 8081
```

## Check JSON

Verifies that the JSON file is valid based on the JSON schema.

Usage:

```shell
rung check json --file <FILE> --template <TEMPLATE>

OPTIONS:
    -f, --file <FILE>            input file to validate
    -t, --template <TEMPLATE>    template file
```

You can get more details with this command:

```shell
rung check json --help
```

Examples:

```shell
run check json \
  -f ./assets/json/example.json \
  -t ./assets/json/example.schema.json
```

## Check Header

Verifies that the file(s) header is matching one or multiple templates.
Typically, used for license header checks in source code files.

Main features:

- single file with multiple templates (matches any single)
- multiple files with single template 
- multiple files with multiple templates

Usage:

```shell
rung check header --file <FILE>... --template <TEMPLATE>...

OPTIONS:
    -f, --file <FILE>...            input file to validate
    -t, --template <TEMPLATE>...    template file
```

You can get more details with this command:

```shell
rung check header --help
```

Examples:

```shell
# using multiple templates
rung check header \
  -f ./assets/files/example-1.ts.txt \
  -t ./assets/templates/template-asf.txt \ 
  -t ./assets/templates/template-mit.txt
  
# using multiple files and templates
rung check header \
  -f ./assets/files/* \
  -t ./assets/templates/*
```
