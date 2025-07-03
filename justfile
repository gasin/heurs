default:
    @just --list

clean:
    rm heurs.db

migrate:
    touch heurs.db
    DATABASE_URL="sqlite://heurs.db" sea-orm-cli migrate fresh --migration-dir crates/database/migration

install:
    cargo install --path crates/cli
    