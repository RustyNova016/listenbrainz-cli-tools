# How to initialize the project

You first need to set your environment file. In the terminal:

```
export SQLX_OFFLINE=true
cargo run cache init-database 
```

This will set the app in "offline" mode, AKA without a database.
Then we run a command that will initialise the database.

You can then create a new `.env` file in the root of the project:

```dotenv
DATABASE_URL=sqlite:<Full path to your debug file>
```

On linux, it should be at `~/.cache/alistral/debug/debug_db.db`.

Close the previous terminal, reopen it, and it will compile against the debug database