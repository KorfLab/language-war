#include <getopt.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <threads.h>
#include <stdint.h>
#include <time.h>

#include "khash.h"
#include "krng.h"

KHASH_MAP_INIT_INT64(m64, int64_t)

#define THREAD_COUNT 4
#define NUMS_PER_THREAD 100

static char *usage = "\
usage: genotype [OPTIONS] <iterations> <depth>\n\
example: genotype 1e9 20\n\
options:\n\
  -e  <float>  err rate [0.1]\n\
  -t  <int>    threads [2]\n\
  -o  <path>   output file [stdout]\n\
  -h           show this message and exit\n\
";

typedef  khash_t(m64)* count_t;

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

	if (argc - optind == 2) {
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

typedef struct {
	int       thread_id;
	program_t program;
	count_t   hom_hash;
	count_t   het_hash;
} ctx_t;


int count_genotypes(void *arg) {
	ctx_t *ctx = (ctx_t *)arg;
	program_t p = ctx->program;
	double limit = p->iterations / p->threads;
	krng_t r;
	uint64_t seed = (uintptr_t)ctx ^ (uint64_t)time(NULL);
	kr_srand_r(&r, seed);

	// homozygous
	count_t hom = kh_init(m64);
	if (!hom) thrd_exit(thrd_error);
	for (int i = 0; i < limit; i++) {
		int count[4] = {0, 0, 0, 0};
		for (int j = 0; j < p->depth; j++) {
			if (kr_drand_r(&r) < p->error) {
				int nt = kr_rand_r(&r) % 3 + 1;
				count[nt]++;
			} else {
				count[0]++;
			}
		}
		int idx = a4idxer(count, p->depth +1);
		int ret;
		khiter_t k = kh_put(m64, hom, idx, &ret);
		if (ret) kh_value(hom, k) = 1;
		else     kh_value(hom, k)++;
	}
	ctx->hom_hash = hom;

	// count homozygous
	count_t het = kh_init(m64);
	if (!het) thrd_exit(thrd_error);
	for (int i = 0; i < limit; i++) {
		int count[4] = {0, 0, 0, 0};
		for (int j = 0; j < p->depth; j++) {
			if (kr_rand_r(&r) % 2 == 0) {
				if (kr_drand_r(&r) < p->error) {
					int nt = kr_rand_r(&r) % 3 + 1; // not mom
					count[nt]++;
				} else {
					count[0]++; // mom
				}
			} else {
				// dad is usually idx:3
				if (kr_drand_r(&r) < p->error) {
					int nt = kr_rand_r(&r) % 3; // not dad
					count[nt]++;
				} else {
					count[3]++; // dad
				}
			}
		}
		int idx = a4idxer(count, p->depth +1);
		int ret;
		khiter_t k = kh_put(m64, het, idx, &ret);
		if (ret) kh_value(het, k) = 1;
		else     kh_value(het, k)++;
	}
	ctx->het_hash = het;

	return thrd_success;
}

count_t hash_union(count_t h1, count_t h2) {
	count_t res = kh_init(m64);
	khiter_t k;
	int ret;

	// 1. Copy all from h1
	for (k = kh_begin(h1); k != kh_end(h1); ++k) {
		if (kh_exist(h1, k)) {
			khiter_t it = kh_put(m64, res, kh_key(h1, k), &ret);
			kh_val(res, it) = kh_val(h1, k);
		}
	}

	// 2. Copy all from h2 (sum values if key already exists)
	for (k = kh_begin(h2); k != kh_end(h2); ++k) {
		if (kh_exist(h2, k)) {
			khiter_t it = kh_put(m64, res, kh_key(h2, k), &ret);
			if (ret == 0) {
				// Key already existed from h1, sum them
				kh_val(res, it) += kh_val(h2, k);
			} else {
				// New key from h2
				kh_val(res, it) = kh_val(h2, k);
			}
		}
	}

	return res;
}


int main(int argc, char **argv) {
	program_t p = proc_cli(argc, argv);
	thrd_t * threads = malloc(sizeof(thrd_t) * THREAD_COUNT);
	ctx_t * ctxs = malloc(sizeof(ctx_t) * THREAD_COUNT);

	// execute threads
	for (int i = 0; i < THREAD_COUNT; i++) {
		ctxs[i].thread_id = i;
		ctxs[i].program = p;
		if (thrd_create(&threads[i], count_genotypes, &ctxs[i]) != thrd_success) {
			return 1;
		}
	}
	for (int i = 0; i < THREAD_COUNT; i++) thrd_join(threads[i], NULL);

	// merge homs
	count_t homs = kh_init(m64);
	for (int i = 0; i < p->threads; i++) {
		count_t hom = ctxs[i].hom_hash;
		if (!hom) continue;
		for (khiter_t k = kh_begin(hom); k != kh_end(hom); k++) {
			if (kh_exist(hom, k)) {
				int ret;
				uint64_t key = kh_key(hom, k);
				int64_t val = kh_val(hom, k);
				khiter_t mk = kh_put(m64, homs, key, &ret);
				if (ret > 0) kh_value(homs, mk) = val;
				else kh_value(homs, mk) += val;
			}
		}
	}
	// merge hets
	count_t hets = kh_init(m64);
	for (int i = 0; i < p->threads; i++) {
		count_t het = ctxs[i].het_hash;
		if (!het) continue;
		for (khiter_t k = kh_begin(het); k != kh_end(het); k++) {
			if (kh_exist(het, k)) {
				int ret;
				uint64_t key = kh_key(het, k);
				int64_t val = kh_val(het, k);
				khiter_t mk = kh_put(m64, hets, key, &ret);
				if (ret > 0) kh_value(hets, mk) = val;
				else kh_value(hets, mk) += val;
			}
		}
	}

	// outout - is bugged
	count_t keys = hash_union(homs, hets);
	for (khiter_t k = kh_begin(keys); k != kh_end(keys); k++) {
		double hom = 0;
		double het = 0;
		if kh_exist(homs, k) hom = kh_val(homs, k);
		if kh_exist(hets, k) het = kh_val(hets, k);
		if (hom == 0 && het == 0) continue;
		uint64_t key = kh_key(keys, k);
		int a, b, c, d;
		idx2abcd(key, p->depth+1, &a, &b, &c, &d);
		printf("%d.%d.%d.%d\t%g\t%g\n", a, b, c, d, hom, het);
	}


	return 0;
}
