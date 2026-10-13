RISCV_PREFIX ?= riscv64-unknown-elf-

CC      := $(RISCV_PREFIX)gcc
OBJDUMP := $(RISCV_PREFIX)objdump
OBJCOPY := $(RISCV_PREFIX)objcopy

CFLAGS   := -march=rv32i -mabi=ilp32
CFLAGS   += -static -nostdlib -nostartfiles -ffreestanding
CFLAGS   += -mcmodel=medany -fvisibility=hidden
CPPFLAGS := -I./env -I./riscv-tests/isa/macros/scalar
LDFLAGS  := -T./riscv-tests/env/p/link.ld

SRCDIR := ./riscv-tests/isa/rv32ui
OBJDIR := ./bin/riscv-tests

PREFIX = rv32ui-p-

EXCLUDES := fence_i.S
EXCLUDES := $(addprefix $(SRCDIR)/, $(EXCLUDES))

SRCS := $(wildcard $(SRCDIR)/*.S)
SRCS := $(filter-out $(EXCLUDES), $(SRCS))

OBJS  := $(patsubst $(SRCDIR)/%.S, $(OBJDIR)/$(PREFIX)%.elf, $(SRCS))
DUMPS := $(patsubst $(SRCDIR)/%.S, $(OBJDIR)/$(PREFIX)%.dump, $(SRCS))
BINS  := $(patsubst $(SRCDIR)/%.S, $(OBJDIR)/$(PREFIX)%.bin, $(SRCS))

all: $(OBJS) $(DUMPS) $(BINS)

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(OBJDIR)/$(PREFIX)%.elf: $(SRCDIR)/%.S | $(OBJDIR)
	$(CC) $(CFLAGS) $(CPPFLAGS) $(LDFLAGS) $< -o $@

$(OBJDIR)/$(PREFIX)%.dump: $(OBJDIR)/$(PREFIX)%.elf
	$(OBJDUMP) -d -M numeric,no-aliases $< > $@

$(OBJDIR)/$(PREFIX)%.bin: $(OBJDIR)/$(PREFIX)%.elf
	$(OBJCOPY) -O binary $< $@

clean:
	rm -rf $(OBJDIR)/*

.PHONY: all clean
