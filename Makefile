install-toolkit:
	cargo install sqlx-cli --no-default-features --features sqlite

setup:
	@cp -n .env.sample .env; \
	sqlx database create --database-url "sqlite:./database.db";
