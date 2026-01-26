# Makefile for Rust project

# Detect the operating system and architecture.

include makefiles/osdetect.mk

# -----------------------------------------------------------------------------
# Variables
# -----------------------------------------------------------------------------

# "Simple expanded" variables (':=')

# PROGRAM_NAME is the name of the GIT repository.
PROGRAM_NAME := $(shell basename `git rev-parse --show-toplevel`)
MAKEFILE_PATH := $(abspath $(firstword $(MAKEFILE_LIST)))
MAKEFILE_DIRECTORY := $(shell dirname $(MAKEFILE_PATH))
TARGET_DIRECTORY := $(MAKEFILE_DIRECTORY)/target
DIST_DIRECTORY := $(MAKEFILE_DIRECTORY)/dist
BUILD_TAG := $(shell git describe --always --tags --abbrev=0  | sed 's/v//')
BUILD_ITERATION := $(shell git log $(BUILD_TAG)..HEAD --oneline | wc -l | sed 's/^ *//')
BUILD_VERSION := $(shell git describe --always --tags --abbrev=0 --dirty  | sed 's/v//')
GIT_REMOTE_URL := $(shell git config --get remote.origin.url)
GIT_REPOSITORY_NAME := $(shell basename `git rev-parse --show-toplevel`)
GIT_VERSION := $(shell git describe --always --tags --long --dirty | sed -e 's/\-0//' -e 's/\-g.......//')
PATH := $(MAKEFILE_DIRECTORY)/bin:$(PATH)


# Conditional assignment. ('?=')
# Can be overridden with "export"
# Example: "export LD_LIBRARY_PATH=/path/to/my/senzing/er/lib"


# Export environment variables.

.EXPORT_ALL_VARIABLES:

# -----------------------------------------------------------------------------
# The first "make" target runs as default.
# -----------------------------------------------------------------------------

.PHONY: default
default: help

# -----------------------------------------------------------------------------
# Operating System / Architecture targets
# -----------------------------------------------------------------------------

-include makefiles/$(OSTYPE).mk
-include makefiles/$(OSTYPE)_$(OSARCH).mk


.PHONY: hello-world
hello-world: hello-world-osarch-specific

# -----------------------------------------------------------------------------
# Dependency management
# -----------------------------------------------------------------------------

.PHONY: dependencies-for-development
dependencies-for-development: dependencies-for-development-osarch-specific
	@sudo npm install -g cspell@latest || true
	@rustup update


.PHONY: dependencies
dependencies:
	@cargo update

# -----------------------------------------------------------------------------
# Setup
# -----------------------------------------------------------------------------

.PHONY: setup
setup:
	@docker run \
		--detach \
		--env SENZING_TOOLS_ENABLE_ALL=true \
		--name senzing-serve-grpc \
		--publish 8261:8261 \
		--rm \
		senzing/serve-grpc
	$(info senzing/serve-grpc running in background.)
	$(info Sleeping to allow grpc server to come up.)
	sleep 3


.PHONY: setup-mutual-tls
setup-mutual-tls:
	@docker run \
		--detach \
		--env SENZING_TOOLS_CLIENT_CA_CERTIFICATE_FILE=/testdata/certificates/certificate-authority/certificate.pem \
		--env SENZING_TOOLS_ENABLE_ALL=true \
		--env SENZING_TOOLS_SERVER_CERTIFICATE_FILE=/testdata/certificates/server/certificate.pem \
		--env SENZING_TOOLS_SERVER_KEY_FILE=/testdata/certificates/server/private_key.pem \
		--name senzing-serve-grpc \
		--publish 8261:8261 \
		--rm \
		--volume $(MAKEFILE_DIRECTORY)/testdata:/testdata \
		senzing/serve-grpc
	$(info senzing/serve-grpc with Mutual TLS running in background.)
	$(info Sleeping to allow grpc server to come up.)
	sleep 3


.PHONY: setup-server-side-tls
setup-server-side-tls:
	@docker run \
		--detach \
		--env SENZING_TOOLS_ENABLE_ALL=true \
		--env SENZING_TOOLS_SERVER_CERTIFICATE_FILE=/testdata/certificates/server/certificate.pem \
		--env SENZING_TOOLS_SERVER_KEY_FILE=/testdata/certificates/server/private_key.pem \
		--name senzing-serve-grpc \
		--publish 8261:8261 \
		--rm \
		--volume $(MAKEFILE_DIRECTORY)/testdata:/testdata \
		senzing/serve-grpc
	$(info senzing/serve-grpc with Server-Side TLS running in background.)
	$(info Sleeping to allow grpc server to come up.)
	sleep 3

# -----------------------------------------------------------------------------
# Lint
# -----------------------------------------------------------------------------

.PHONY: lint
lint:  cspell

# -----------------------------------------------------------------------------
# Build
# -----------------------------------------------------------------------------


.PHONY: build
build:
	cargo build

# -----------------------------------------------------------------------------
# Run
# -----------------------------------------------------------------------------

.PHONY: run
run:
	cargo run

# -----------------------------------------------------------------------------
# Test
# -----------------------------------------------------------------------------

.PHONY: test
test:
	cargo test


.PHONY: test-verbose
test-verbose:
	cargo test -- --nocapture


.PHONY: test-mutual-tls
test-mutual-tls: export SENZING_TOOLS_SERVER_CA_CERTIFICATE_FILE=$(MAKEFILE_DIRECTORY)/testdata/certificates/certificate-authority/certificate.pem
test-mutual-tls: export SENZING_TOOLS_CLIENT_CERTIFICATE_FILE=$(MAKEFILE_DIRECTORY)/testdata/certificates/client/certificate.pem
test-mutual-tls: export SENZING_TOOLS_CLIENT_KEY_FILE=$(MAKEFILE_DIRECTORY)/testdata/certificates/client/private_key.pem
test-mutual-tls:
	cargo test -- --nocapture


.PHONY: test-mutual-tls-encrypted-key
test-mutual-tls-encrypted-key: export SENZING_TOOLS_SERVER_CA_CERTIFICATE_FILE=$(MAKEFILE_DIRECTORY)/testdata/certificates/certificate-authority/certificate.pem
test-mutual-tls-encrypted-key: export SENZING_TOOLS_CLIENT_CERTIFICATE_FILE=$(MAKEFILE_DIRECTORY)/testdata/certificates/client/certificate.pem
test-mutual-tls-encrypted-key: export SENZING_TOOLS_CLIENT_KEY_FILE=$(MAKEFILE_DIRECTORY)/testdata/certificates/client/private_key_encrypted.pem
test-mutual-tls-encrypted-key: export SENZING_TOOLS_CLIENT_KEY_PASSPHRASE=Passw0rd
test-mutual-tls-encrypted-key:
	cargo test -- --nocapture


.PHONY: test-server-side-tls
test-server-side-tls: export SENZING_TOOLS_SERVER_CA_CERTIFICATE_FILE=$(MAKEFILE_DIRECTORY)/testdata/certificates/certificate-authority/certificate.pem
test-server-side-tls:
	cargo test -- --nocapture

# -----------------------------------------------------------------------------
# Coverage
# -----------------------------------------------------------------------------

.PHONY: coverage
coverage:


.PHONY: check-coverage
check-coverage:

# -----------------------------------------------------------------------------
# Documentation
# -----------------------------------------------------------------------------

.PHONY: documentation
documentation: documentation-osarch-specific

# -----------------------------------------------------------------------------
# Clean
# -----------------------------------------------------------------------------

.PHONY: clean
clean:
	@docker rm --force senzing-serve-grpc || true

# -----------------------------------------------------------------------------
# Utility targets
# -----------------------------------------------------------------------------

.PHONY: help
help:
	$(info Build $(PROGRAM_NAME) version $(BUILD_VERSION)-$(BUILD_ITERATION))
	$(info Makefile targets:)
	@$(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$@$$' | xargs


.PHONY: print-make-variables
print-make-variables:
	@$(foreach V,$(sort $(.VARIABLES)), \
		$(if $(filter-out environment% default automatic, \
		$(origin $V)),$(info $V=$($V) ($(value $V)))))

# -----------------------------------------------------------------------------
# Specific programs
# -----------------------------------------------------------------------------

.PHONY: cspell
cspell:
	@cspell lint --dot .
