.PHONY: all clean fmt fmt-check install linter pre-commit

all:
	@cargo ledger build nanos
	@cargo ledger build nanosplus
	@cargo ledger build nanox
	@cp -vf target/nanos/release/app.hex assets/app_nanos.hex
	@cp -vf target/nanosplus/release/app.hex assets/app_nanosplus.hex
	@cp -vf target/nanox/release/app.hex assets/app_nanox.hex

clean:
	@rm -rvf target

fmt:
	@cargo fmt

fmt-check:
	@cargo fmt -- --check

install: all
	@ledgerctl delete Vara
	@cd assets
	@ledgerctl install app_nanosplus.json

linter:
	@cargo clippy --all-features -- -D warnings

pre-commit: fmt linter all
