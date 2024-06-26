# Making sure to have a local environment (will be loaded with cargo lambda watch via start)
include .env
export

default: help

help:
		@echo "Please use \033[32mmake \033[32m<target>\033[39m where <target> is one of"
		@echo "  \033[32m help \033[39m               Shows this help"
		@echo "  \033[32m start \033[39m              Start the setup with node, postgres, redis and rust lambda watch"
		@echo "  \033[32m stop \033[39m               Stops the setup"
		@echo "  \033[32m build \033[39m              Rebuilds the setup"
		@echo "  \033[32m volume_prune \033[39m       Removing all volumes and downing the project"
		@echo "  \033[32m deploy \033[39m       	  Deploying lambda"
		@echo "  \033[32m migration_up \033[39m       Running diesel migrations"
		@echo "  \033[32m migration_redo \033[39m     Running diesel migrations down + up"
		@echo "  \033[32m readme \033[39m             Shows some help"
		@echo "  \033[32m test \033[39m               Running all unittest via nextest"
		@echo "  \033[32m test_coverage \033[39m      Find a coverage report in %targetdir%/debug/coverage/index.html"

list:
	$(MAKE) help

start:
	docker-compose up -d
	cargo lambda watch --env-vars SENTRY_DSN=$(SENTRY_DSN),SENTRY_SAMPLE_RATE=$(SENTRY_SAMPLE_RATE),DATABASE_URL=$(DATABASE_URL),MAX_DATABASE_CONNECTIONS=$(MAX_DATABASE_CONNECTIONS),APP_SECRET=$(APP_SECRET),SESSION_LIFETIME=$(SESSION_LIFETIME),REDIS_DSN=$(REDIS_DSN)

stop:
	docker-compose down

build:
	docker-compose up -d --build

volume_prune:
	docker-compose down -v

deploy:
	cargo lambda build --release
	cargo lambda deploy -p rust -r eu-central-1

migration_up:
	DATABASE_URL=$(DATABASE_URL) MAX_DATABASE_CONNECTIONS=$(MAX_DATABASE_CONNECTIONS) diesel migration run

migration_redo:
	DATABASE_URL=$(DATABASE_URL) MAX_DATABASE_CONNECTIONS=$(MAX_DATABASE_CONNECTIONS) diesel migration redo

readme:
	@echo "\033[32m cargo run\033[39m to run the (rust) server"
	@echo "\033[32m docker-compose exec node sh\033[39m and than \033[32mnpm run dev\033[39m to start the vite web server"

test_coverage:
	RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="web_app-%p-%m.profraw" cargo clean;
	RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="web_app-%p-%m.profraw" cargo test;
	# Running the actual coverage report
	grcov . --binary-path ./target/debug/ -s . -t html --ignore-not-existing --excl-start "#\[cfg\(test\)\]" --excl-line "#\[derive\(" --ignore "/*" -o ./target/debug/coverage/;
	rm web_app*.profraw;

test:
	cargo nextest run