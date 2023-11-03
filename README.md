# mdbook-abbr

mdBook preprocessor that expands defined abbreviations.

## Installation

```bash
cargo install mdbook-abbr
```

## Configuration

```toml
[preprocessor.abbr]
# This enables the preprocessor

[preprocessor.abbr.list]
# List the abbrevations and their expansions
API = "Application Programming Interface"
GQL = "GraphQL"
```
