CC=gcc
CFLAGS=-Wall
LDFLAGS=-lm

.PHONY: all
all: main

.PHONY: clean
clean:
	$(RM) *~ *.o c_call_rust

OBJECTS=main.o
c_call_rust : $(OBJECTS)
	$(CC) $(CFLAGS) $(OBJECTS) -o c_call_rust -L../helloproto/target/debug/ -ltestadd -static
