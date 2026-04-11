#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>

#include "kseq.h"
#include "mylib.h"

int main(int argc, char **argv) {
	mylib_greeting(stdout, "kmers");
	return 0;
}

