#LINKER_ARGS		:= -mcpu=cortex-m0 -mthumb -TMKL43Z256xxx4_ram.ld -nostdlib -Wl,--fatal-warnings -Wl,-uabort # -fuse-ld=gold
LINKER_ARGS		:= -Tqemu-cortex-m3.ld -nostdlib -Wl,--fatal-warnings -Wl,--no-undefined -Wl,--no-allow-shlib-undefined # -fuse-ld=gold
SYSROOT			:= sysroot
RUSTC_SYSROOT	:= $(PWD)/$(SYSROOT)
RUSTC			:= $(PWD)/$(SYSROOT)/rustc-sysroot
TARGET_SYSROOT	:= $(RUSTC_SYSROOT)/lib/rustlib/$(TARGET)
#TARGET			:= cortex-m0
TARGET			:= thumbv7m-qemu-eabi
PREFIX			:= arm-none-eabi-
LD				:= $(PREFIX)gcc
GDB				:= $(PREFIX)gdb
PROFILE			:= debug

export RUSTC_SYSROOT
export RUSTC

all: $(TARGET_SYSROOT)
	@#cargo rustc -v --target $(TARGET) -- -C link-args="$(LINKER_ARGS)" -C linker=$(LD)
	@#cargo rustc --target $(TARGET) -- -Z no-landing-pads
	@#cargo rustc --example hello -v --target $(TARGET) -- -C link-args="$(LINKER_ARGS)" -C linker=$(LD)
	@#cargo rustc --example hello --target $(TARGET) -- -C link-args="$(LINKER_ARGS)" -C linker=$(LD) -Z no-landing-pads
	@cargo rustc --example hello --target $(TARGET) -- -C link-args="$(LINKER_ARGS)" -C linker=$(LD) -Z no-landing-pads -g
	@#cargo rustc --release -v --example hello --target $(TARGET) -- -C link-args="$(LINKER_ARGS)" -C linker=$(LD) -Z no-landing-pads -C opt-level=3 -C lto -v -g

run: all
	$(GDB) target/$(TARGET)/$(PROFILE)/cirno -ex "target remote localhost:3333" -ex "load"

qemu: all
	#qemu-system-arm -machine versatilepb -cpu cortex-m3 -nographic -semihosting -semihosting-config enable=on,target=native -s -S &
	#qemu-system-arm -machine lm3s811evb -cpu cortex-m3 -nographic -semihosting -semihosting-config enable=on,target=native -s -S -kernel target/$(TARGET)/debug/examples/hello &
	#arm-none-eabi-gdb target/$(TARGET)/$(PROFILE)/examples/hello -ex 'target remote localhost:1234' -ex load -ex 'set variable $$sp = &__stack_start'
	qemu-system-arm -machine lm3s6965evb -cpu cortex-m3 -nographic -semihosting -semihosting-config enable=on,target=native -s -kernel target/$(TARGET)/$(PROFILE)/examples/hello

qemu-gdb: all
	setsid qemu-system-arm -machine lm3s6965evb -cpu cortex-m3 -nographic -semihosting -semihosting-config enable=on,target=native -s -kernel target/$(TARGET)/$(PROFILE)/examples/hello -S &
	arm-none-eabi-gdb target/$(TARGET)/$(PROFILE)/examples/hello -ex 'target remote localhost:1234'

$(TARGET_SYSROOT):
	env TARGET="$(TARGET)" CFLAGS="$(LINKER_ARGS)" RUSTC_FLAGS="-Z no-landing-pads -C opt-level=3 -g" ./download

.PHONY: all run
