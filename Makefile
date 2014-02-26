CC=rustc
CFLAGS=
LDFLAGS=-L lib

.PHONY : doc

all: clean compile

compile:
	mkdir -p bin
	$(CC) $(CFLAGS) -o bin/rust-story $(LDFLAGS) src/main.rs

veyron: CFLAGS += -O -Z time-passes
veyron: all


debug: CFLAGS += -g -Z time-passes
debug: compile

doc:
	rustdoc $(LDFLAGS) src/main.rs

run:
	bin/rust-story
clean:
	rm -f bin/**
