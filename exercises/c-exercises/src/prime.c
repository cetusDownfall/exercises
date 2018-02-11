#include <stdio.h>

int * primes(int num);
int main() {
	return 0;
}
int * primes(int num) {
	int *nums, *primes, i, prime;
	nums = (int *) malloc(sizeof(int) * (num - 2));
	if (nums == NULL) return NULL;
	for (i = 0; i < num; i++) *(num + i) = 1;

