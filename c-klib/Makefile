# Makefile for c-klib

LIB_DIR  := lib
PROGRAMS := dust kmers genotype exons params
SUBDIRS  := $(LIB_DIR) $(PROGRAMS)

.PHONY: all clean $(SUBDIRS)

all: $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@

clean:
	@for dir in $(SUBDIRS); do \
		$(MAKE) -C $$dir clean; \
	done

