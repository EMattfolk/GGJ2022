TARGET=main.lua
SOURCE=othello.sy

run: $(TARGET)
	love .

$(TARGET): $(SOURCE) *.sy | sylt
	../target/debug/sylt -o $@ $<

sylt:
	cd .. && cargo b

clean:
	rm -rf $(TARGET)

