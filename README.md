# rung: (Ru)st + A(ng)ular

Useful command-line tools for Angular projects written in Rust.

Commands:

- List
- Serve
- Check JSON
- Check Header

## Getting Help

Use the `--help` argument to get more details about the program or specific command:

```shell
rung --help
rung <command> --help
```

## Commands

### List

Provides listing of the contents of the `angular.json` file.

```shell
USAGE:
    rung ls [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <PATH>    [default: angular.json]

SUBCOMMANDS:
    apps    List all applications
    help    Prints this message or the help of the given subcommand(s)
    libs    List all libraries
```

Examples:

```shell
# list all projects and libraries
rung ls

# list all applications
rung ls apps

# list all libraries
rung ls libs
```

By default, the CLI expects the `angular.json` file to be in the current directory.
It is also possible to provide a custom path:

```shell
rung ls libs -c ./assets/angular/angular.json
```

### Serve

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

### Check JSON

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

### Check Header

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

## License

Rung is primarily distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for more details.
