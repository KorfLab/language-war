#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>

#include "kseq.h"
#include "mylib.h"

static char *usage = "\
usage: dust [OPTIONS] <fasta>\n\
options:\n\
  -s, --size <int>       window size [20]\n\
  -e, --entropy <float>  entropy threhold [1.4]\n\
  -l, --lower            soft mask\n\
  -h, --help             show this message and exit\n\
";


int main(int argc, char **argv) {
	int size = 20;
	double entropy = 1.4;
	int lower = 0;
	int opt;
	
	static struct option long_options [] = {
		{"size",    optional_argument, 0, 's'},
		{"entropy", optional_argument, 0, 'e'},
		{"lower",   no_argument,       0, 'l'},
		{"help",    no_argument,       0, 'h'},
		{0, 0, 0, 0}
	};
	
    while ((opt = getopt_long(argc, argv, "vdf:h", long_options, NULL)) != -1) {
		switch (opt) {
			case 's': size = atoi(optarg); break;
			case 'e': entropy = atof(optarg); break;
			case 'l': lower = 1; break;
			case 'h': printf("%s", usage); return 0;
			default: printf("%s", usage); return 1;
		}
	}

	if (optind >= argc) {
		fprintf(stderr, "Error: Missing <command>\n %s", usage);
		return 1;
	}

	char *fasta = argv[optind];
	fprintf(stderr, "processing %s\n", fasta);

	return 0;
}
