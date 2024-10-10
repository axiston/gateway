# Makefile for sea-orm-cli entity generation.

# Variables
SEA_ORM_CLI = sea-orm-cli
DATABASE_URL = postgresql://usr:pwd@localhost:5432/db

# Default target
.PHONY: generate
generate:
	$(SEA_ORM_CLI) generate entity \
		--database-url $(DATABASE_URL) \
		--database-schema public \
		--output-dir ./entity \
		--date-time-crate time \
		--with-serde both \
		--verbose

# Helper to clean for a next re-run (if necessary).
.PHONY: clean
clean:
	rm -rf ./entity
	@echo "Clean-up target (if needed)."

# Optionally, include a run target for demonstration
.PHONY: run
run: generate
	@echo "Entity generation complete!"

.PHONY: install
install:
	@echo "Installation complete!"
