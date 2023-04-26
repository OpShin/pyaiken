use pyo3::{prelude::*, exceptions::PyValueError};
use miette::{IntoDiagnostic, Report};
use uplc::{
    ast::{DeBruijn, Program, NamedDeBruijn, Term, Name},
    parser, machine::cost_model::ExBudget,
};
use std::fmt;

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

pub fn _uplc_unflat(
    cbor: String
) -> Result<String, AikenError> {
    let mut cbor_buffer = Vec::new();
    let mut flat_buffer = Vec::new();

    let program = Program::<DeBruijn>::from_hex(cbor.trim(), &mut cbor_buffer, &mut flat_buffer)
            .into_diagnostic()?;

    let program: Program<Name> = program.try_into().into_diagnostic()?;

    let pretty = program.to_pretty();

    return Ok(pretty);
}

#[pyfunction]
pub fn unflat(
    code: String
) -> PyResult<String> {
    return Ok(_uplc_unflat(code)?);
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

    let mut res = program.eval(budget);
    let cost = res.cost();
    let logs = res.logs();
    let term = res.result();

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

// Module registration

fn register_module_uplc(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "uplc")?;
    m.add_function(wrap_pyfunction!(flat, m)?)?;
    m.add_function(wrap_pyfunction!(unflat, m)?)?;
    m.add_function(wrap_pyfunction!(eval, m)?)?;
    parent_module.add_submodule(m)?;
    Ok(())
}


/// A Python module implemented in Rust.
#[pymodule]
fn pyaiken(_py: Python, m: &PyModule) -> PyResult<()> {
    register_module_uplc(_py, m)?;
    Ok(())
}
