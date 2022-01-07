VERSION := $(shell cat Cargo.toml | grep version | head -1 | cut -d\" -f2 | xargs)
CARGOFLAGS :=
TARGET := debug
OUTFILES := $(shell cat Cargo.toml | grep name | head -1 | cut -d\" -f2 | xargs)
RUBISH := $(wildcard package/*) $(wildcard target*/*)
.PHONY: package

all: build package
all-release: build-release 
	$(MAKE) package TARGET=release

build:
	cargo build $(CARGOFLAGS)

build-release:
	$(MAKE) build CARGOFLAGS=--release

package:
	mkdir -p package
	tar -czvf package/dupfile-$(VERSION).tgz target/$(TARGET)/$(OUTFILES)

clean:
	rm -rdf $(RUBISH)