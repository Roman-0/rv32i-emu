add-addi.bin: add-addi.s
	riscv32-unknown-linux-gnu-gcc -Wl,-Ttext=0x0 -nostdlib -o add-addi add-addi.s
	riscv32-unknown-linux-gnu-objcopy -O binary add-addi add-addi.bin

clean:
	rm -f add-addi
	rm -f add-addi.bin
