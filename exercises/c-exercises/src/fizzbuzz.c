#include <stdio.h>

int main() {
	int i;
	for(i = 1; i <= 100; printf("\n"), i++)
		if ((i % 5) != 0 && (i % 3) != 0) printf("%d", i);
		else {
			if ((i % 3) == 0) printf("Fizz");
			if ((i % 5) == 0) printf("Buzz");
		}
	return 0;
}
