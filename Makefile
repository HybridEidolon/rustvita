TITLE := RUST01337
TARGET := RustVita

.PHONY: target/vita/debug/rustvita.elf

rustvita.vpk: rustvita.velf
	vita-make-fself -s $< build/eboot.bin
	vita-mksfoex -s TITLE_ID=$(TITLE) "$(TARGET)" param.sfo
	cp -f param.sfo build/sce_sys/param.sfo
	cd build
	7z a -tzip rustvita.vpk -r sce_sys/* eboot.bin
	cd ..
	mv build/rustvita.vpk rustvita.vpk

rustvita.velf: target/vita/debug/rustvita.elf
	cp $< rustvita.unstripped.elf
	arm-vita-eabi-strip -g $<
	vita-elf-create $< $@

target/vita/debug/rustvita.elf:
	xargo build
