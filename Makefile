TITLE := RUST01337
TARGET := RustVita

.PHONY: target/vita/debug/rustvita.elf clean doc

all: rustvita.vpk

doc:
	xargo doc

clean:
	xargo clean
	rm -f *.elf
	rm -f *.velf
	rm -f *.vpk
	rm -f build/eboot.bin
	rm -f build/sce_sys/param.sfo

rustvita.vpk: rustvita.velf
	vita-make-fself -s $< build/eboot.bin
	vita-mksfoex -s TITLE_ID=$(TITLE) "$(TARGET)" param.sfo
	cp -f param.sfo build/sce_sys/param.sfo
	7z a -tzip rustvita.vpk -bd -r ./build/sce_sys ./build/eboot.bin

rustvita.velf: target/vita/debug/rustvita.elf
	cp $< rustvita.unstripped.elf
	arm-vita-eabi-strip -g $<
	vita-elf-create $< $@ $(VITASDK)/share/db.json sceshaccgc.json

target/vita/debug/rustvita.elf:
	xargo build

