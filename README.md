# env-checker-cli

A lightweight command-line tool that checks whether the environment variables
required by your application are present and non-empty. It can also load values
from a `.env` file using [`dotenvy`](https://crates.io/crates/dotenvy).

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo

## Required variables

The required variable names are currently defined in `src/main.rs`:

| Variable | Purpose |
| --- | --- |
| `DATABASE_URL` | Database connection string |
| `API_KEY` | Application or service API key |
| `PORT` | Port used by the application |

The checker verifies that each variable exists and is not empty or whitespace
only. It does not validate the format or value of a variable.

## Usage

Run from the project directory:

```sh
cargo run
```

The program checks variables available in the process environment and searches
for a `.env` file starting in the working directory. For example, create a
local `.env` file with:

```dotenv
DATABASE_URL=postgres://localhost/myapp
API_KEY=replace-with-your-key
PORT=8080
```

Alternatively, set the variables in your shell before running the program.

To build an executable:

```sh
cargo build --release
```

The executable is written to `target/release/env-checker-cli` (or
`target\release\env-checker-cli.exe` on Windows).

## Results and exit status

The checker reports the status of each required variable. It exits with status
`0` when all required variables are set and non-empty, and status `1` if any
are missing or empty. The names of missing or empty variables are included in
the error output.

## Changing the required variables

Edit the `REQUIRED_VARS` list in `src/main.rs` to add or remove variable names,
then run the tool again.

## Security

Do not commit real credentials or secret values. Keep your local `.env` file
out of version control and use your deployment platform's secret management for
production values.

## License

This project is licensed under the MIT License.