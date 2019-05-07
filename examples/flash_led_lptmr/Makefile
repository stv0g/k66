PROG = flash_led_ptmr

TARGET = thumbv7em-none-eabihf
MODE = release

all: $(PROG).hex

$(PROG): src/main.rs
	cargo build --$(MODE)

$(PROG).bin: $(PROG)
	cargo objcopy --bin $(PROG) --$(MODE) -- --output-target=binary $@

$(PROG).hex: $(PROG).bin
	# cargo/llvm-binutils can not output Intel Hex files yet...
	# So we have to fall back to a GCC binutils here.
	arm-none-eabi-objcopy -I binary -O ihex $^ $@

flash: $(PROG).hex
	teensy_loader_cli -w $^

BOARD_TAG = TEENSY36
REMOTE_HOST = rock64@rck
REMOTE_UPLOAD_CMD = teensy_loader_cli -v -s -w --mcu=$(BOARD_TAG)

remote_flash: $(PROG).hex
	ssh $(REMOTE_HOST) 'TMP=$$(mktemp); cat > $${TMP}; $(REMOTE_UPLOAD_CMD) $${TMP}; rm $${TMP}' < $^

.PHONY: flash all
