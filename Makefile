CC=rustc
CFLAGS=
LDFLAGS=-L lib

.PHONY : doc

all: clean compile

compile:
	$(CC) $(CFLAGS) -o bin/rust-story $(LDFLAGS) src/main.rs

debug: CFLAGS += -g -Z time-passes
debug: compile

doc:
	rustdoc $(LDFLAGS) src/main.rs

run:
	bin/rust-story
clean:
	rm -f bin/**
