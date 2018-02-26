#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int interpret(char * c, char * cmp, char * bf_in, FILE * bf_out);
void bf_run(char * cm_ptr, char * bf_in, char * out_path);
int main(int argc, char ** argv) {
	char *bf_out, bf_alpha[8] = {',', '.', '+', '-', '>', '<', '[', ']'}, cm, *comms, *out, *bf_in;
	int flen, ind;
	FILE * bf_cm = argc > 1? fopen(argv[1], "r"):NULL;
	if (argc > 2) bf_in = argv[2];
	else {
		puts("Please input data for program.");
		scanf("%s", bf_in);
	}
	bf_out = argv[3];
	if (bf_cm == NULL) return 1;
	for (flen = 0; cm != EOF; cm = fgetc(bf_cm)) if (memchr(bf_alpha, cm, 8)) flen++;
	comms = (char *)malloc(sizeof(char) * (flen + 1));
	if (comms == NULL) return 1;
	comms[flen] = '\0';
	freopen (argv[1], "r", bf_cm);
	for (cm = fgetc(bf_cm), ind = 0; cm != EOF; cm = fgetc(bf_cm)) if (memchr(bf_alpha,cm,8)) comms[ind++] = cm;
	printf("%s", comms);
	bf_run (comms, bf_in, bf_out);
	return 0;
}
void bf_run(char * cm_ptr, char * bf_in, char * out_path) {
	FILE* bf_out = fopen(out_path, "w");
	char cell[30000] = {0}, *c;
	c = cell;
	while ((c - cell) >= 0 && (c - cell) < 30000 && interpret(c, cm_ptr, bf_in, bf_out));
	fclose(bf_out);
}
int interpret(char * c, char * cmp, char * bf_in, FILE * bf_out) {
	char tmp;
	tmp = *cmp;
	switch (tmp) {
		case ',': *c = *(bf_in=='\0'?bf_in:bf_in++); break;
		case '.': fprintf(bf_out, "%c", *c); break;
		case '+': (*c)++; break;
		case '-': (*c)--; break;
		case '>': (c++); break;
		case '<': (c--); break;
		case '[': if (!*c) {
				  int sct;
				  sct = 0;
				  do {
					  cmp++;
					  if (*cmp == '[') sct++;
					  if (*cmp == ']') sct--;
				  } while (sct > 0 || *cmp != ']');
			  } break;
		case ']': if (!*c) {
				  int lct;
				  lct = 0;
				  do {
					  cmp--;
					  if (*cmp == ']') lct++;
					  if (*cmp == '[') lct--;
				  } while (lct > 0 || *cmp != '[');
			  } break;
	} 
	return *(cmp++);
}
