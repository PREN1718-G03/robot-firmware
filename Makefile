BIN_NAME     := robot-firmware
TARGET       := riscv32imac-unknown-none
OPENOCD_CFG  := riscv-crates/lofive-openocd.cfg
BAUD_RATE    := 115200
TTY          := /dev/ttyUSB1
LOG_FILE  := /tmp/robot-firmware.log

TARGET_DIR   := $(abspath ./target/$(TARGET)/debug)
TARGET_BIN   := $(TARGET_DIR)/$(BIN_NAME)

build:
	xargo build --target $(TARGET)

clean:
	xargo clean

readelf:
	llvm-readelf -a -h -s -r -symbols $(TARGET_BIN)

objdump:
	llvm-objdump -d -S $(TARGET_BIN)

size:
	llvm-size $(TARGET_BIN)

gdb:
	riscv32-unknown-elf-gdb $(TARGET_BIN)

openocd:
	rm $(LOG_FILE)
	# setup tty
	stty -F $(TTY) $(BAUD_RATE) sane -opost -brkint -icrnl -isig -icanon -iexten -echo
	# start openocd
	openocd -f $(OPENOCD_CFG) 1>>$(LOG_FILE) &
	# open tty
	cat $(TTY) 1>>$(LOG_FILE) &
	# print logfile to stdout
	tail -qF $(LOG_FILE) | stcat -e $(TARGET_BIN)
	# close tty
	fg
	# stop openocd
	fg

flash: build
	openocd -f $(OPENOCD_CFG) \
		-c "flash protect 0 64 last off; program ${TARGET_BIN}; resume 0x20400000; exit"

.PHONY: build clean readelf objdump size gdb openocd flash
