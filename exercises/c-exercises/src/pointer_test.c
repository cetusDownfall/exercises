#include <stdlib.h>
#include <stdio.h>

int main() {
	char test[12] = "World! Hello";
	char* point = &test[0];
	for (int i = 0; i < 12; i++) {
		printf("%c", *point);
		printf("\n");
		point++;
	}
	return 0;
}
