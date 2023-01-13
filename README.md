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

```

### Building

In case you need to build this package from source, install Python3 and Rust and proceed as follows.

```bash
git clone https://github.com/ImperatorLang/pyaiken
cd pyaiken
python3 -m venv .env
source .env/bin/activate  # or in whichever environment you want to have it installed
pip install manturin
manturin build
```

The package will be installed in the active python environment.