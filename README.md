pyaiken
=======
[![CI](https://github.com/OpShin/pyaiken/actions/workflows/CI.yml/badge.svg)](https://github.com/OpShin/pyaiken/actions/workflows/CI.yml)
[![Build Status](https://app.travis-ci.com/OpShin/pyaiken.svg?branch=master)](https://app.travis-ci.com/OpShin/pyaiken)
[![PyPI version](https://badge.fury.io/py/pyaiken.svg)](https://pypi.org/project/pyaiken/)
[![PyPI - Status](https://img.shields.io/pypi/status/pyaiken.svg)](https://pypi.org/project/pyaiken/)

This package supplies python bindings for the package [aiken](https://github.com/aiken-lang/aiken).
The bindings are added on a per-need basis, currently only serving the development of [opshin](https://github.com/opshin)


### Installation

Install python3. Then run the following command.

```bash
python3 -m pip install pyaiken
```

### Usage


```python

from pyaiken import uplc

# Tools for handling UPLC

### uplc.flat
# Print the hex of the CBOR encoded flat-encoding of the program in UPLC textual notation
code = uplc.flat("(program 1.0.0 (con data #01))")

print(code)
# prints "490100004c0101010001"

### uplc.unflat
# Print the UPLC in textual notation from the hex of the CBOR encoded flat-encoding
code = uplc.unflat("490100004c0101010001")

print(code)
# prints "(program 1.0.0 (con data #01))"

### uplc.eval
# Evaluate a UPLC program with the given arguments (all in textual representation) and cpu and memory budget (optional, in this order)
# Returns either computed UPLC value on success or thrown error on failure,
# logs generated through trace
# and the consumed cpu and memory steps
((suc, err), logs, (cpu, mem)) = uplc.eval("(program 1.0.0 (lam x x))", ["(con data #01)"], 1000000, None)

print((suc, err), logs, (cpu, mem))
# prints "('(con data #01)', None), [], (92100, 500)"

```

### Building

In case you need to build this package from source, install Python3 and Rust and proceed as follows.

```bash
git clone https://github.com/OpShin/pyaiken
cd pyaiken
python3 -m venv .env
source .env/bin/activate  # or in whichever environment you want to have it installed
pip install maturin
maturin build
```

The package will be installed in the active python environment.
