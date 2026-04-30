#include <getopt.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "khash.h"
#include "krng.h"
#include "mylib.h"

KHASH_MAP_INIT_INT(num_counts, uint32_t)

static char *usage = "\
usage: genotype [OPTIONS] <iterations> <depth>\n\
example: genotype 1e9 20\n\
options:\n\
  -e  <float>  err rate [0.1]\n\
  -t  <int>    threads [2]\n\
  -o  <path>   output file [stdout]\n\
  -h           show this message and exit\n\
";

typedef struct ProgramParameters {
	double iterations;
	int    depth;
	double error;
	int    threads;
	FILE  *output;
} ProgramParameters, *program_t;

program_t proc_cli(int argc, char **argv) {
	program_t p = malloc(sizeof(ProgramParameters));
	p->iterations = 0;
	p->error = 0.1;
	p->depth = 0;
	p->threads = 2;
	p->output = stdout;
	int opt;

    while ((opt = getopt(argc, argv, "e:t:o:h")) != -1) {
		switch (opt) {
			case 'e': p->error = atof(optarg); break;
			case 't': p->threads = atoi(optarg); break;
			case 'o': p->output = fopen(optarg, "w"); break;
			case 'h': printf("%s", usage); exit(0);
			default: printf("%s", usage); exit(1);
		}
	}

	if (argc == 3) {
		p->iterations = atof(argv[optind]);
		p->depth = atoi(argv[optind+1]);
	} else {
		fprintf(stderr, "%s", usage);
		exit(1);
	}

	return p;
}

static int a4idxer(int a[4], int base) {
	// selection sort is fast with 4 elements
	for (int i = 0; i < 3; i++) {
		for (int j = i + 1; j < 4; j++) {
			if (a[i] < a[j]) {
				int t = a[i];
				a[i] = a[j];
				a[j] = t;
			}
		}
	}
	
	int idx = 
		a[0] * pow(base, 3) + 
		a[1] * pow(base, 2) +
		a[2] * pow(base, 1) + 
		a[3] * 1;
	
	return idx;
} 

static void idx2abcd(int n, int base, int *a, int *b, int *c, int *d) {
	*a = (n / (base * base * base)) % base;
	*b = (n / (base * base)) % base;
	*c = (n / base) % base;
	*d = n % base;
}

static khash_t(num_counts)* count_homozygous(program_t p, krng_t *rng) {
	khash_t(num_counts) *h = kh_init(num_counts);
	double limit = p->iterations / p->threads;
	for (int i = 0; i < limit; i++) {
		int count[4] = {0, 0, 0, 0};
		for (int j = 0; j < p->depth; j++) {
			if (kr_drand_r(rng) < p->error) {
				int nt = kr_rand_r(rng) % 3 + 1;
				count[nt]++;
			} else {
				count[0]++;
			}
		}
		int idx = a4idxer(count, p->depth +1);
		int ret;
		khiter_t k = kh_put(num_counts, h, idx, &ret);
		if (ret) kh_value(h, k) = 1;
		else     kh_value(h, k)++;
	}
	
	return h;
}

static khash_t(num_counts)* count_heterozygous(program_t p, krng_t *rng) {
	khash_t(num_counts) *h = kh_init(num_counts);
	double limit = p->iterations / p->threads;
	for (int i = 0; i < limit; i++) {
		int count[4] = {0, 0, 0, 0};
		for (int j = 0; j < p->depth; j++) {
			if (kr_rand_r(rng) % 2 == 0) {
				if (kr_drand_r(rng) < p->error) {
					int nt = kr_rand_r(rng) % 3 + 1; // not mom
					count[nt]++;
				} else {
					count[0]++; // mom
				}
			} else {
				// dad is usually idx:3
				if (kr_drand_r(rng) < p->error) {
					int nt = kr_rand_r(rng) % 3; // not dad
					count[nt]++;
				} else {
					count[3]++; // dad
				}
			}
		}
		int idx = a4idxer(count, p->depth +1);
		int ret;
		khiter_t k = kh_put(num_counts, h, idx, &ret);
		if (ret) kh_value(h, k) = 1;
		else     kh_value(h, k)++;
	}
	
	return h;
}

/*
static size_t get_khash_mem(khash_t(num_counts) *h) {
	size_t n = h->n_buckets;
	size_t mem = sizeof(*h);
	mem += n * sizeof(khint32_t);
	mem += n * sizeof(int);
	return mem;
}
*/


int main(int argc, char **argv) {
	program_t p = proc_cli(argc, argv);
	krng_t rng;
	kr_srand_r(&rng, (uint64_t)time(NULL));
	
	printf("homozygous\n");
	khash_t(num_counts) *hom = count_homozygous(p, &rng);
	for (khiter_t it = kh_begin(hom); it != kh_end(hom); it++) {
		if (!kh_exist(hom, it)) continue;
		int k = kh_key(hom, it);
		int v = kh_val(hom, it);
		int a, b, c, d;
		idx2abcd(k, p->depth+1, &a, &b, &c, &d);
		
		printf("%d %d %d %d: %d\n", a, b, c, d, v);
	}
	
	khash_t(num_counts) *het = count_heterozygous(p, &rng);
	printf("heterozygous\n");
	for (khiter_t it = kh_begin(het); it != kh_end(het); it++) {
		if (!kh_exist(het, it)) continue;
		int k = kh_key(het, it);
		int v = kh_val(het, it);
		int a, b, c, d;
		idx2abcd(k, p->depth+1, &a, &b, &c, &d);
		
		printf("%d %d %d %d: %d\n", a, b, c, d, v);
	}
	
	//printf("%d %d\n", get_khash_mem(hom), get_khash_mem(het));
	
	return 0;
}
