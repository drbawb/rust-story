RUSTC=rustc
RUST_FLAGS=
LDFLAGS=-L lib

.PHONY : all clean doc 

all: clean compile

compile:
	mkdir -p bin
	$(RUSTC) $(RUST_FLAGS) -o bin/rust-story $(LDFLAGS) src/main.rs

veyron: RUST_FLAGS += -O -Z time-passes -Z lto
veyron: all


debug: RUST_FLAGS += -g -Z time-passes
debug: compile

deps:	
	git submodule update --init	
	mkdir -p lib	
	rm -f lib/libsdl2*	
	cd lib/rust-sdl2; make clean && make
	cp lib/rust-sdl2/build/lib/libsdl2* lib/

doc:
	rustdoc $(LDFLAGS) src/main.rs

run:
	bin/rust-story
clean:
	rm -f bin/**
