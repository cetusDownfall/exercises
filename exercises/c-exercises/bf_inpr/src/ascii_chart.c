#include <stdio.h>

int main(int argc, char **argv) {
	unsigned char num;
	if (argc > 1) num = (unsigned char) *argv[1];
	else num = 0;
	if (argc > 1) printf("%c: %d", num, num);
	else while (num < 255) printf("%c: %d\n", num, num++);
	return 0;
}
