#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>

#include "mylib.h"

int main(int argc, char **argv) {
	mylib_greeting(stdout, "genotype");
	return 0;
}
