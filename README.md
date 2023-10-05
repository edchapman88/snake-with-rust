# Get started
- `python -m venv .venv`
- `source .venv/bin/activate`
- `pip install -e '.[dev]'` (install the `snake` package in as an 'editable' package, including the `dev` optional dependencies to install `maturin` and `pytest`)

# Test
- `cargo test`
- `pytest` (or `pytest -rP` to include stdout)

# Develop
- `maturin develop` rebuilds the python wheel from the Rust source code.

# Starting a new project
A lot of the initialisation can be covered my `maturin` if starting a new project:
- `mkdir new-project`
- `cd new-project`
- `python -m venv .venv`
- `source .venv/bin/activate`
- `pip install maturin`
- `maturin init`

See the [docs](https://pyo3.rs/main/getting_started)