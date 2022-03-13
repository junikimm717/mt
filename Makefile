TARGET=./target/release/mt
PREFIX=/usr/local

UNAME := $(shell uname)
CC := gcc
ifeq ($(UNAME), Linux)
EXTRA=--target=x86_64-unknown-linux-musl
TARGET=./target/x86_64-unknown-linux-musl/release/mt
endif

$(TARGET):
	cargo build --release $(EXTRA)
	chmod +x $(TARGET)

install: $(TARGET)
	cp $(TARGET) $(PREFIX)/bin/mt

uninstall:
	rm -rf $(TARGET)/bin/mt

clean:
	rm -rf $(TARGET)

.PHONY: install uninstall
