TARGET=./target/release/mt
PREFIX=/usr/local

$(TARGET):
	cargo build --release
	chmod +x $(TARGET)

install: $(TARGET)
	cp $(TARGET) $(PREFIX)/bin/mt

uninstall:
	rm -rf $(TARGET)/bin/mt

clean:
	rm -rf $(TARGET)

.PHONY: install uninstall
