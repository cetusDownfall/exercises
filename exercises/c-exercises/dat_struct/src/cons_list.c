#include <stdio.h>

struct node { struct node *cdr; char car; };

int main() { 
	struct node h, i, 
		    w, o, r, l, d, 
		    *curr;
	h.cdr = &i; i.cdr = NULL;
	w.cdr = &o; o.cdr = &r; r.cdr = &l; l.cdr = &d; d.cdr = NULL;
	h.car = 'H'; i.car = 'i';
	w.car = 'W'; o.car = 'o'; r.car = 'r'; l.car = 'l'; d.car = 'd';
	for (curr = &h; curr != NULL; curr = curr->cdr)
		printf("%c", curr->car);
	printf(", ");
	for (curr = &w; curr != NULL; curr = curr->cdr)
		printf("%c", curr->car);
	return 0; 
}
