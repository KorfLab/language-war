# Makefile for kmers

CC      := gcc
CFLAGS  := -std=c11 -Wall -Wextra -I../lib/include
LDFLAGS := -L../lib -lmylib -lsqlite3 -lm -lz

TARGET  := kmers
SRCS    := main.c
OBJ     := main.o

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(OBJ) -o $@ $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)
