#!/usr/bin/make

CC = arm-none-eabi-gcc
CFLAGS =  -mcpu=arm1176jzf-s -fpic -ffreestanding -std=gnu99 -O2 -Wall -Wextra
ASM_FLAGS = -mcpu=arm1176jzf-s -fpic -ffreestanding
OBJ = boot.o kernel.o

kernel.elf: ${OBJ}
	${CC} -T linker.ld -o $@ -ffreestanding -O2 -nostdlib ${OBJ}

boot.o: boot.S
	${CC} ${ASM_FLAGS} -c $< -o $@

kernel.o: kernel.c
	${CC} ${CFLAGS} -c $< -o $@

clean: 
	rm -f *.o *.elf

.PHONY: clean
