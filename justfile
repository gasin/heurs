default:
    @just --list

migrate:
    touch heurs.db
    DATABASE_URL="sqlite://heurs.db" sea-orm-cli migrate refresh --migration-dir crates/database/migration

install-cli:
    cargo install --path crates/cli
    