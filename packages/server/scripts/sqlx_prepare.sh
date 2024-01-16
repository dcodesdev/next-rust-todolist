# Generates types for SQLX queries
# Runs `cargo sqlx prepare`

pnpm types

cargo sqlx prepare

git add .sqlx/*