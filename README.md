# L2 Opcodes Validator
Simple program to validate Ethereum bytecode and check for illegal opcodes
This program validates Ethereum bytecode and checks for illegal opcodes. It takes a JSON file as input and outputs a list of all illegal opcodes found in the bytecode.

## Usage

`cargo run -- --path <path_to_json_file>`

### Options

`--path`: The path to the JSON file containing the bytecode.

`--illegal-opcodes`: A list of illegal opcodes to check for. The default value is `SELFDESTRUCT`.

### Example

`cargo run --path example.json --illegal-opcodes DUP2 SHA3`

Output:

```
Looking for the following illegal opcodes: ["DUP2", "SHA3"]
No illegal opcodes found
```


### Panic
If any illegal opcodes are found in the bytecode, the program will panic.
