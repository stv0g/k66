PROG = flash_led_ptmr

TARGET = thumbv7em-none-eabihf
MODE = debug

PROG_ELF = target/$(TARGET)/$(MODE)/$(PROG)

all: $(PROG_ELF)

$(PROG_ELF): src/main.rs
	cargo build

flash: $(PROG_ELF)
	openocd -f openocd.cfg -c "program $^ verify reset exit"

debug: $(PROG_ELF)
	openocd -f openocd.cfg &
	arm-none-eabi-gdb $^ -ex "target remote localhost:3333"
	killall openocd

.PHONY: flash all
