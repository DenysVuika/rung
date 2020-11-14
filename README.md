# rung

Rust tools for Angular projects

## Check Header

Verifies that the file(s) header is matching one or multiple templates.
Typically, used for license header checks in source code files.

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
rung check header -f ./example-1.ts.txt \
  -t ./templates/template-asf.txt \ 
  -t ./templates/template-mit.txt
  
# using multiple files and templates
rung check header -f ./test/example-* \
  -t ./test/template-*
```
