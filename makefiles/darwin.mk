# Makefile extensions for darwin.

# -----------------------------------------------------------------------------
# Variables
# -----------------------------------------------------------------------------

SENZING_DIR ?= /opt/senzing/er
SENZING_TOOLS_SENZING_DIRECTORY ?= $(SENZING_DIR)
LD_LIBRARY_PATH ?= $(SENZING_TOOLS_SENZING_DIRECTORY)/lib:$(SENZING_TOOLS_SENZING_DIRECTORY)/lib/macos
DYLD_LIBRARY_PATH := $(LD_LIBRARY_PATH)

# -----------------------------------------------------------------------------
# Makefile targets supported only by this platform.
# -----------------------------------------------------------------------------

.PHONY: only-darwin
only-darwin:
	$(info Only darwin has this Makefile target.)
