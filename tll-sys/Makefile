BINDGEN_OPTIONS := --no-doc-comments --no-prepend-enum-name
BINDGEN_OPTIONS += --impl-debug
BINDGEN_OPTIONS += --raw-line "\#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]"

ifeq ($(strip $(shell bindgen --help | grep -- --size_t-is-usize)), --size_t-is-usize)
	BINDGEN_OPTIONS += --size_t-is-usize
endif

TLL := $(HOME)/src/tll
INCLUDE := -I$(TLL)/src

all: src/logger.rs src/config.rs src/channel.rs src/processor.rs src/processor_loop.rs src/scheme.rs

src/logger.rs: $(TLL)/src/tll/logger.h
	bindgen -o $@ $(BINDGEN_OPTIONS) \
		--allowlist-type tll_logger.* --allowlist-function tll_logger.* \
		--blocklist-type 'tll_config_.*' \
		--raw-line 'use crate::config::tll_config_t;' \
		$< -- $(INCLUDE)

src/config.rs: $(TLL)/src/tll/config.h
	bindgen -o $@ $(BINDGEN_OPTIONS) --allowlist-type tll_config.* --allowlist-function tll_config.* $< -- $(INCLUDE)

src/scheme.rs: $(TLL)/src/tll/scheme.h
	bindgen -o $@ $(BINDGEN_OPTIONS) --allowlist-type 'tll_scheme.*' --allowlist-function tll_scheme.* $< -- $(INCLUDE)

src/channel.rs: $(TLL)/src/tll/channel/module.h
	bindgen -o $@ $(BINDGEN_OPTIONS) \
		--allowlist-type 'tll_channel.*' --allowlist-function tll_channel.* \
		--allowlist-var 'TLL_CHANNEL_.*' \
		--allowlist-type 'tll_msg_.*' \
		--allowlist-type 'tll_message_.*' \
		--blocklist-type 'tll_(config|scheme|stat|logger)_.*' \
		--raw-line 'use crate::config::tll_config_t;' \
		--raw-line 'use crate::logger::{tll_logger_t, tll_logger_level_t};' \
		--raw-line 'use crate::scheme::tll_scheme_t;' \
		--raw-line 'use crate::stat::*;' \
		$< -- $(INCLUDE)

src/processor.rs: $(TLL)/src/tll/processor.h
	bindgen -o $@ $(BINDGEN_OPTIONS) \
		--allowlist-type 'tll_processor.*' --allowlist-function tll_processor.* \
		--blocklist-type 'tll_(config|channel)_.*' \
		--blocklist-type 'tll_processor_loop_t' \
		--raw-line 'use crate::channel::*;' \
		--raw-line 'use crate::processor_loop::*;' \
		$< -- $(INCLUDE)

src/processor_loop.rs: $(TLL)/src/tll/processor/loop.h
	bindgen -o $@ $(BINDGEN_OPTIONS) \
		--allowlist-type 'tll_processor.*' --allowlist-function tll_processor.* \
		--blocklist-type 'tll_(config|channel)_.*' \
		--raw-line 'use crate::channel::*;' \
		--raw-line 'use crate::config::tll_config_t;' \
		$< -- $(INCLUDE)
