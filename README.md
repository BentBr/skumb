## Local setup and installation
Copy `.env.example` to `.env` and adjust the values to your needs and see Makefile for available commands.

```bash
cp .env.example .env
make help
```

### Rust
### Diesel
- `diesel migration generate create_to_do_items` Using diesel to create a new migration
- Running the migration: `diesel migration run`
- Undoing and running again the migration: `diesel migration redo`
### Docker (compose)
### AWS CLI
### Cargo Lambda
Install: \
`brew tap cargo-lambda/cargo-lambda` \
`brew install cargo-lambda`




Run
`cargo lambda watch`

Deploy
`cargo lambda deploy -p rust -r eu-central-1`


https://9hzofcbnk1.execute-api.eu-central-1.amazonaws.com/
https://9hzofcbnk1.execute-api.eu-central-1.amazonaws.com/test


Todos:
- local db
- aws db
- local FE (node)
- aws FE deploy