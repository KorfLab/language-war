# Makefile for mylib

CC      := gcc
CFLAGS  := -std=c11 -Wall -Wextra -Wpedantic -O3 -Iinclude
LDFLAGS := -lptread -ldl

LIB_NAME := libmylib.a
SRC_DIR  := src
OBJ_DIR  := obj
INCLUDE  := include/mylib.h

SRCS := $(wildcard $(SRC_DIR)/*.c)
OBJS := $(SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)

.PHONY: all clean

all: $(LIB_NAME)

$(LIB_NAME): $(OBJS)
	ar rcs $@ $(OBJS)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c $(INCLUDE) | $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ_DIR):
	mkdir -p $@

clean:
	rm -rf $(OBJ_DIR) $(LIB_NAME)
