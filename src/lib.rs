use pyo3::{exceptions::PyValueError, prelude::*};
use thiserror::Error;

// pyclass requires no lifetime parameters, no generic parameters, and must implement Send
// (unless annotated with #[pyclass(unsendable)]).
#[pyclass]
struct Snakeplace {
    pub injection: String,
    pub repeats: Option<usize>,
}

#[pymethods]
impl Snakeplace {
    // Declaring a python constructor (only pythons `__new__` method can be specified, `__init__` is
    // not available)
    #[new]
    pub fn new(injection: String, repeats: Option<usize>) -> PyResult<Self> {
        if injection == "Python" {
            Err(SnakeError::BadInjection)?
        }
        Ok(Snakeplace { injection, repeats })
    }

    pub fn transform(&self, source: String) -> String {
        let n = self.repeats.unwrap_or(1);
        source.replace("Python", &self.injection.repeat(n))
    }
}

#[pymodule]
fn snake(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Snakeplace>()?;
    Ok(())
}

#[derive(Error, Debug)]
enum SnakeError {
    #[error("Must inject a word other than 'Python', how about 'Rust'!")]
    BadInjection,
}

impl From<SnakeError> for PyErr {
    fn from(value: SnakeError) -> Self {
        match value {
            SnakeError::BadInjection => PyValueError::new_err(value.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let snake = Snakeplace::new("Rust".to_string(), None).unwrap();
        assert_eq!(
            snake.transform("Hello Python".to_string()),
            "Hello Rust".to_string()
        );
    }
}
