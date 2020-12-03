# rung: (Ru)st + A(ng)ular

Useful command-line tools for Angular projects written in Rust.

Commands:

- List
- New Application
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

### New Application

Creates a new Angular application with [Angular CLI].

Requires an [Angular CLI] to be installed.

```shell
USAGE:
    rung new [OPTIONS] <name>

ARGS:
    <name>    The name of the new workspace and initial project.

OPTIONS:
    -d, --directory <DIR>    The directory name to create the workspace in.
```

Differences to running `ng new <name>` command directly:

- does not install dependencies automatically by default
- does not configure Git repository by default

Examples:

```shell
# creates new application `app1` in the current directory
rung new app1

# creates a new application `app2` in the `/tmp/apps` directory 
rung new app2 -d /tmp/apps 
```

### Serve

Runs a lightweight web server.

```shell
USAGE:
    rung serve [FLAGS] [OPTIONS] <dir>

ARGS:
    <dir>    Target directory
    
FLAGS:
    -o, --open       Opens the url in default browser.

OPTIONS:
    -h, --host <HOST>    Host address [default: 127.0.0.1]
    -p, --port <PORT>    Port number [default: 8080]
```

Examples:

```shell
# serves application at http://localhost:8081
rung serve ./dist/app1 -p 8081

# serves the application and opens default system browser
rung serve ./dist/app1 -p 8081 --open
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

```shell
USAGE:
    rung check header --file <FILE> --template <TEMPLATE>...

OPTIONS:
    -f, --file <FILE>               input file
    -t, --template <TEMPLATE>...    template file

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

[Angular CLI]: https://angular.io/cli
