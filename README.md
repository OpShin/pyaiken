pyaiken
=======

This package supplies python bindings for the package [aiken](https://github.com/aiken-lang/aiken).
The bindings are added on a per-need basis, currently only serving the development of [eopsin](https://github.com/ImperatorLang/eopsin)


### Installation

Install python3. Then run the following command.

```bash
python3 -m pip install pyaiken
```

### Usage


```python

from pyaiken import uplc, script_address

# Tools for handling UPLC

### uplc.flat
# Print the hex of the CBOR encoded flat-encoding of the program in UPLC textual notation
code = uplc.flat("(program 1.0.0 (con data #01))")

print(code)
# prints "490100004c0101010001"

### uplc.eval
# Evaluate a UPLC program with the given arguments (all in textual representation) and cpu and memory budget (optional, in this order)
# Returns either computed UPLC value on success or thrown error on failure,
# logs generated through trace
# and the consumed cpu and memory steps
((suc, err), logs, (cpu, mem)) = uplc.eval("(program 1.0.0 (lam x x))", ["(con data #01)"], 1000000, None)

print((suc, err), logs, (cpu, mem))
# prints "('(con data #01)', None), [], (9907900, 13999500)"


### script_address.build_mainnet | script_address.build_test
# Compute the bech32 representations of a PlutusV2 script, based on the CBOR hex of the flat encoded script.
addr_mainnet = script_address.build_mainnet(code)
addr_testnet = script_address.build_testnet(code)

print(addr_mainnet)
# prints "addr1wx3x3tt88a9c44fxvv03w879v5ye98hzayq0yv4x7jhr0lc7pge2l"
print(addr_testnet)
# prints "addr_test1wz3x3tt88a9c44fxvv03w879v5ye98hzayq0yv4x7jhr0lc9fu996"