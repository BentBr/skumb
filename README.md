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


https://9hzofcbnk1.execute-api.eu-central-1.amazonaws.com/
https://9hzofcbnk1.execute-api.eu-central-1.amazonaws.com/test


Todos:
- clippy strict
- add tests
- add chat entity
- add BE chat uri + web transport connection
- add auth layer to endpoints (current an auth user can query all endpoints)
