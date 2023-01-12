use pyo3::{prelude::*, exceptions::PyValueError};
use miette::{IntoDiagnostic, Report};
use uplc::{
    ast::{DeBruijn, Program, NamedDeBruijn, Term},
    parser, machine::cost_model::ExBudget,
};
use std::fmt;
use pallas::ledger::{addresses::Address, primitives::babbage};
use pallas_traverse::ComputeHash;

// UPLC submodule

#[derive(Debug)]
pub struct AikenError {
    rep: Report
}

impl std::error::Error for AikenError {}

impl fmt::Display for AikenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::convert::From<Report> for AikenError {
    fn from(err: Report) -> AikenError {
        AikenError { rep: err }
    }
}

impl std::convert::From<AikenError> for PyErr {
    fn from(err: AikenError) -> PyErr {
        PyValueError::new_err(err.rep.to_string())
    }
}




pub fn _uplc_flat(
    code: String
) -> Result<String, AikenError> {
    let program = parser::program(&code).into_diagnostic()?;

    let program = Program::<DeBruijn>::try_from(program).into_diagnostic()?;
    let cbor = program.to_hex().into_diagnostic()?;
    return Ok(cbor);
}

#[pyfunction]
pub fn flat(
    code: String
) -> PyResult<String> {
    return Ok(_uplc_flat(code)?);
}

type EvalRes = ((Option<String>, Option<String>), Vec<String>, (i64, i64));

pub fn _uplc_eval(
    code: String,
    args: Vec<String>,
    budget: (Option<i64>, Option<i64>),
) -> Result<EvalRes, AikenError> {
    let mut program = {
        let prog = parser::program(&code).into_diagnostic()?;

        Program::<NamedDeBruijn>::try_from(prog).into_diagnostic()?
    };

    for arg in args {
        let term: Term<NamedDeBruijn> = parser::term(&arg)
            .into_diagnostic()?
            .try_into()
            .into_diagnostic()?;

        program = program.apply_term(&term);
    }
    let cpu = match budget.0 {
        Some(budget) => budget,
        None => ExBudget::default().cpu
    };
    let mem = match budget.1 {
        Some(budget) => budget,
        None => ExBudget::default().mem
    };

    let budget = ExBudget { mem: mem, cpu: cpu };

    let (term, cost, logs) = program.eval(budget);

    return Ok((
        match term {
            Ok(term) => {
                (Some(term.to_string()), None)
            }
            Err(err) => {
                (None, Some(err.to_string()))
            }
        },
        logs,
        (cost.cpu, cost.mem),
    ))
}

#[pyfunction]
pub fn eval(
    code: String,
    args: Vec<String>,
    cpubudget: Option<i64>,
    membudget: Option<i64>,
) -> PyResult<EvalRes> {
    return Ok(_uplc_eval(code, args.try_into()?, (cpubudget, membudget))?);
}

// Address submodule

pub fn _mainnet_bytes() -> Vec<u8> {
   return vec![0b01110001];
}

#[pyfunction]
pub fn mainnet_bytes() -> PyResult<Vec<u8>> {
    return Ok(_mainnet_bytes());
}

pub fn _testnet_bytes() -> Vec<u8> {
    return vec![0b01110000];
}

#[pyfunction]
pub fn testnet_bytes() -> PyResult<Vec<u8>> {
    return Ok(_testnet_bytes());
}

pub fn _script_address_build(
    cbor_hex: String,
    _network_bytes: Vec<u8>,
) -> Result<String, AikenError> {
    let cbor = hex::decode(cbor_hex).into_diagnostic()?;

    // Create mainnet and testnet addresses
    let plutus_script = babbage::PlutusV2Script(cbor.into());

    let hash = plutus_script.compute_hash();

    // mainnet
    let mut network_bytes: Vec<u8> = _network_bytes;
    network_bytes.extend(hash.iter());

    let addr = Address::from_bytes(&network_bytes)
        .unwrap()
        .to_bech32()
        .unwrap();

    return Ok(addr);
}


#[pyfunction]
pub fn build(
    cbor_hex: String,
    network_bytes: Vec<u8>,
) -> PyResult<String> {
    return Ok(_script_address_build(cbor_hex, network_bytes)?);
}

#[pyfunction]
pub fn build_mainnet(
    cbor_hex: String,
) -> PyResult<String> {
    return Ok(_script_address_build(cbor_hex, _mainnet_bytes())?);
}

#[pyfunction]
pub fn build_testnet(
    cbor_hex: String,
) -> PyResult<String> {
    return Ok(_script_address_build(cbor_hex, _testnet_bytes())?);
}

// Module registration

fn register_module_uplc(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "uplc")?;
    m.add_function(wrap_pyfunction!(flat, m)?)?;
    m.add_function(wrap_pyfunction!(eval, m)?)?;
    parent_module.add_submodule(m)?;
    Ok(())
}


fn register_module_address(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "script_address")?;
    m.add_function(wrap_pyfunction!(build, m)?)?;
    m.add_function(wrap_pyfunction!(build_mainnet, m)?)?;
    m.add_function(wrap_pyfunction!(build_testnet, m)?)?;
    m.add_function(wrap_pyfunction!(mainnet_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(testnet_bytes, m)?)?;
    parent_module.add_submodule(m)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyaiken(_py: Python, m: &PyModule) -> PyResult<()> {
    register_module_uplc(_py, m)?;
    register_module_address(_py, m)?;
    Ok(())
}
