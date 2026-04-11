#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>

#include "mylib.h"

int main(int argc, char **argv) {
	mylib_greeting(stdout, "params");
	return 0;
}
