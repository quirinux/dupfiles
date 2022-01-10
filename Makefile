VERSION := $(shell cat Cargo.toml | grep version | head -1 | cut -d\" -f2 | xargs)
CARGOFLAGS :=
TARGET := debug
OUTFILES := $(shell cat Cargo.toml | grep name | head -1 | cut -d\" -f2 | xargs)
RUBISH := package target
ARCH := linux-x86_64
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
	tar -czvf package/dupfile-$(VERSION)-$(ARCH).tgz -C target/$(TARGET) $(OUTFILES) 

clean:
	rm -rdf $(RUBISH)

version:
	@echo $(VERSION)
