.SUFFIXES:

ifeq ($(strip $(DEVKITARM)),)
$(error "Please set DEVKITARM in your environment. export DEVKITARM=<path to>devkitARM")
endif

TOPDIR ?= $(CURDIR)
include $(DEVKITARM)/3ds_rules

TARGET  := $(notdir $(CURDIR))
PLGINFO := settings.plgInfo

BUILD   := build
INCLUDES := includes
SOURCES := sources

#---------------------------------------------------------------------------------
# Rust configuration
#---------------------------------------------------------------------------------
RUST_TARGET := armv6k-nintendo-3ds
RUST_PROFILE := release
RUST_DIR := $(TOPDIR)/target/$(RUST_TARGET)/$(RUST_PROFILE)
RUST_LIB := $(RUST_DIR)/libpokemon_bank_offline.a

#---------------------------------------------------------------------------------
# options for code generation
#---------------------------------------------------------------------------------
ARCH := -march=armv6k -mtune=mpcore -mfloat-abi=hard -mtp=soft

CFLAGS := $(ARCH) -Os -mword-relocations \
          -fomit-frame-pointer -ffunction-sections -fno-strict-aliasing

CFLAGS += $(INCLUDE) -D__3DS__

CXXFLAGS := $(CFLAGS) -fno-rtti -fno-exceptions -std=gnu++11

ASFLAGS := $(ARCH)

LDFLAGS := -T $(TOPDIR)/3gx.ld $(ARCH) -Os \
           -Wl,--gc-sections,--strip-discarded,--strip-debug,-z,noexecstack \
           -L$(RUST_DIR)

LIBS := -lpokemon_bank_offline -lctru
LIBDIRS := $(CTRULIB) $(PORTLIBS)

#---------------------------------------------------------------------------------
# no real need to edit anything past this point unless you need to add additional
# rules for different file extensions
#---------------------------------------------------------------------------------
ifneq ($(BUILD),$(notdir $(CURDIR)))
#---------------------------------------------------------------------------------

export OUTPUT := plugin
export TOPDIR := $(CURDIR)
export VPATH := $(foreach dir,$(SOURCES),$(CURDIR)/$(dir)) \
                $(foreach dir,$(DATA),$(CURDIR)/$(dir))

export DEPSDIR := $(CURDIR)/$(BUILD)

CFILES  := $(foreach dir,$(SOURCES),$(notdir $(wildcard $(dir)/*.c)))
CPPFILES := $(foreach dir,$(SOURCES),$(notdir $(wildcard $(dir)/*.cpp)))
SFILES  := $(foreach dir,$(SOURCES),$(notdir $(wildcard $(dir)/*.s)))

export LD := $(CXX)
export OFILES := $(CPPFILES:.cpp=.o) $(CFILES:.c=.o) $(SFILES:.s=.o)

export INCLUDE := $(foreach dir,$(INCLUDES),-I $(CURDIR)/$(dir) ) \
                  $(foreach dir,$(LIBDIRS),-I $(dir)/include) \
                  -I $(CURDIR)/$(BUILD)

export LIBPATHS := $(foreach dir,$(LIBDIRS),-L $(dir)/lib)

.PHONY: all rust clean re lint

#---------------------------------------------------------------------------------
# Build Rust first.
#
# Cargo handles Rust's dependency tracking, so we deliberately make this
# phony. Cargo will do nothing when the Rust sources are already up to date.
#---------------------------------------------------------------------------------
all: $(BUILD)

rust:
	@echo "==> Building Rust..."
	@cargo build --release -Z build-std=core,alloc --target $(RUST_TARGET)

#---------------------------------------------------------------------------------
# Enter the normal devkitARM build directory.
#---------------------------------------------------------------------------------
$(BUILD):
	@mkdir -p $@
	@$(MAKE) --no-print-directory -C $(BUILD) -f $(CURDIR)/Makefile

#---------------------------------------------------------------------------------
# Make sure Rust has been built before the normal build starts.
#---------------------------------------------------------------------------------
$(BUILD): rust

#---------------------------------------------------------------------------------
clean:
	@echo "==> Cleaning..."
	@cargo clean
	@rm -rf $(BUILD) $(OUTPUT).3gx $(OUTPUT).elf

re: clean all

lint:
	@cargo clippy --release -Z build-std=core,alloc --target $(RUST_TARGET)

#---------------------------------------------------------------------------------

else

DEPENDS := $(OFILES:.o=.d)

#---------------------------------------------------------------------------------
# Main targets
#
# Rust is built by the parent Make invocation before this recursive make runs.
# The explicit Rust library dependency also ensures the linker knows that the
# archive is an input to the final binary.
#---------------------------------------------------------------------------------
$(OUTPUT).3gx: $(OUTPUT).elf $(RUST_LIB)

$(OUTPUT).elf: $(OFILES) $(RUST_LIB)

#---------------------------------------------------------------------------------
# You need a rule like this for each extension you use as binary data
#---------------------------------------------------------------------------------
%.bin.o: %.bin
	@echo $(notdir $<)
	@$(bin2o)

#---------------------------------------------------------------------------------
.PRECIOUS: %.elf

%.3gx: %.elf
	@echo creating $(notdir $@)
	@3gxtool -s $(word 1, $^) $(TOPDIR)/$(PLGINFO) $@

#---------------------------------------------------------------------------------

-include $(DEPENDS)

#---------------------------------------------------------------------------------
endif
