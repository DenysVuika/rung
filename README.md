# rung

Rust tools for Angular projects

Commands:

- Check JSON
- Check Header

## Help

Use the `--help` argument to get more details about the program or specific command:

```shell
rung --help
rung <command> --help
```

## Check JSON

Verifies that the JSON file is valid based on the JSON schema.

Command Format:

```shell
rung check json --file <FILE> --template <TEMPLATE>
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

Command format:

```shell
rung check header --file <FILE>... --template <TEMPLATE>...
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
