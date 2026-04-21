#include <getopt.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <zlib.h>

#include "kseq.h"
#include "khash.h"
#include "mylib.h"

KSEQ_INIT(gzFile, gzread)
KHASH_MAP_INIT_STR(kcount, uint32_t)

static char *usage = "\
usage: kmers [-a] <fasta> <k>\n\
options:\n\
  -a         count anti-parallel\n\
  -h         show this message and exit\n\
";

int main(int argc, char **argv) {
	int anti = 0;
	int opt;

	// CLI
	while ((opt = getopt(argc, argv, "ah")) != -1) {
		switch (opt) {
			case 'a': anti = 1; break;
			case 'h': printf("%s", usage); exit(0);
			default:
		}
	}
	if (argc != 3) {
		printf("%s", usage);
		exit(1);
	}
	gzFile fp = gzopen(argv[1], "r");
	int k = atoi(argv[2]);

	/// Init
	kseq_t *rec = kseq_init(fp);
	int n;
	khash_t(kcount) *h = kh_init(kcount);
	int absent;

	// Main Loop
	while ((n = kseq_read(rec)) >= 0) {
		for (uint32_t i = 0; i < rec->seq.l -k+1; i++) {
			char *kmer = strndup(&rec->seq.s[i], k);
			khiter_t iter = kh_put(kcount, h, kmer, &absent);
			if (absent) {
				kh_val(h, iter) = 1;
			} else {
				kh_val(h, iter)++;
				free(kmer);
			}
		}
	}

	// Output
	for (khiter_t it = kh_begin(h); it != kh_end(h); it++) {
		if (!kh_exist(h, it)) continue;
		printf("%s: %u\n", kh_key(h, it), kh_val(h, it));
	}

	// Cleanup
	for (khiter_t it = kh_begin(h); it != kh_end(h); it++) {
		if (kh_exist(h, it)) free((char*)kh_key(h, it));
	}
	kh_destroy(kcount, h);
	kseq_destroy(rec);
	gzclose(fp);

	return 0;

}
