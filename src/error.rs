#[derive(Debug)]
pub enum RuntimeError {
    RedeclaredProcedure { name: String },
    ProcedureNotFound { name: String },
    VariableNotFound { name: String },
    ArgCountMismatch { expected: usize },
    TypeMismatch { expected: String },
    Other(String),
}
