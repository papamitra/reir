#!/usr/bin/make

CC = arm-none-eabi-gcc
CFLAGS =  -mcpu=arm1176jzf-s -fpic -ffreestanding -std=gnu99 -O2 -Wall -Wextra
ASM_FLAGS = -mcpu=arm1176jzf-s -fpic -ffreestanding
OBJ = boot.o main.o context_switch.o

kernel.elf: ${OBJ}
	${CC} -T linker.ld -o $@ -ffreestanding -O2 -nostdlib ${OBJ}

boot.o: boot.S
	${CC} ${ASM_FLAGS} -c $< -o $@

context_switch.o: context_switch.S
	${CC} ${ASM_FLAGS} -c $< -o $@

main.o : main.rs
	rustc --target arm-unknown-linux-gnueabi --emit=obj $<

clean: 
	rm -f *.o *.elf

.PHONY: clean
