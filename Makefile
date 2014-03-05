CC=rustc
CFLAGS=
LDFLAGS=-L lib

.PHONY : all clean doc 

all: clean compile

compile:
	mkdir -p bin
	$(CC) $(CFLAGS) -o bin/rust-story $(LDFLAGS) src/main.rs

veyron: CFLAGS += -O -Z time-passes
veyron: all


debug: CFLAGS += -g -Z time-passes
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
