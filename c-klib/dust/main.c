#include <getopt.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <zlib.h>

#include "kseq.h"
#include "mylib.h"

KSEQ_INIT(gzFile, gzread)

static char *usage = "\
usage: dust [OPTIONS] <fasta>\n\
options:\n\
  -s  <int>    window size [20]\n\
  -e  <float>  entropy threhold [1.4]\n\
  -w  <int>    line wrap length [60]\n\
  -l           soft mask\n\
  -h           show this message and exit\n\
";

typedef struct ProgramParameters {
	int size;
	double entropy;
	int wrap;
	int lower;
	char *fasta;
} ProgramParameters, *program_t;

program_t proc_cli(int argc, char **argv) {
	program_t p = malloc(sizeof(ProgramParameters));
	p->size = 20;
	p->entropy = 1.4;
	p->wrap = 60;
	p->lower = 0;
	p->fasta = NULL;
	int opt;

    while ((opt = getopt(argc, argv, "s:e:w:lh")) != -1) {
		switch (opt) {
			case 's': p->size = atoi(optarg); break;
			case 'e': p->entropy = atof(optarg); break;
			case 'w': p->wrap = atoi(optarg); break;
			case 'l': p->lower = 1; break;
			case 'h': printf("%s", usage); exit(0);
			default: printf("%s", usage); exit(1);
		}
	}

	if (optind < argc) {
		p->fasta = argv[optind];
	} else {
		fprintf(stderr, "Error: Missing <command>\n %s", usage);
		exit(1);
	}

	return p;
}

static double entropy(int a, int c, int g, int t) {
	double h = 0;
	int total = a + c + g + t;
	double pa = (double)a / (double)total;
	double pc = (double)c / (double)total;
	double pg = (double)g / (double)total;
	double pt = (double)t / (double)total;
	if (a != 0) h -= pa * log(pa);
	if (c != 0) h -= pc * log(pc);
	if (g != 0) h -= pg * log(pg);
	if (t != 0) h -= pt * log(pt);
	return h / log(2);
}

static void mask_seq(char *seq, int offset, program_t p) {
	if (p->lower) {
		for (int i = offset; i < offset + p->size; i++)
			if (seq[i] < 'Z') seq[i] += 32;
	} else {
		for (int i = offset; i < offset + p->size; i++) seq[i] = 'N';
	}
}

int main(int argc, char **argv) {
	program_t p = proc_cli(argc, argv);
	gzFile   fp = gzopen(p->fasta, "r");
	kseq_t *rec = kseq_init(fp);
	int n;

	while ((n = kseq_read(rec)) >= 0) {
		char *mask = strdup(rec->seq.s);
		int a = 0, c = 0, g = 0, t = 0;

		// first window
		for (int i = 0; i < p->size; i++) {
			switch (rec->seq.s[i]) {
				case 'A': a++; break;
				case 'C': c++; break;
				case 'G': g++; break;
				case 'T': t++; break;
			}
		}
		if (entropy(a, c, g, t) < p->entropy) mask_seq(mask, 0, p);

		// subsequent windows
		for (int i = 1; i < (int)rec->seq.l - p->size + 1; i++) {
			char off = rec->seq.s[i-1];
			char on  = rec->seq.s[i + p->size -1];
			switch (off) {
				case 'A': a--; break;
				case 'C': c--; break;
				case 'G': g--; break;
				case 'T': t--; break;
			}
			switch (on) {
				case 'A': a++; break;
				case 'C': c++; break;
				case 'G': g++; break;
				case 'T': t++; break;
			}
			if (entropy(a, c, g, t) < p->entropy) mask_seq(mask, i, p);
		}

		// output
		printf(">%s %s\n", rec->name.s, rec->comment.s);
		int len = strlen(mask);
		for (int i = 0; i < len; i += p->wrap)
			printf("%.*s\n", p->wrap, mask + i);

		// cleanup
		free(mask);
	}

	kseq_destroy(rec);
	gzclose(fp);

	return 0;
}
