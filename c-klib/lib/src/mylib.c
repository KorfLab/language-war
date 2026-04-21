/****************************************************************************\
 mylib.c
\****************************************************************************/

#include "mylib.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void mylib_greeting(FILE *fp, const char *text) {
	fprintf(fp, "mylib: %s\n", text);
}
