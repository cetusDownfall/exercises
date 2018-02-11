#include <stdio.h>

int height = 10;

void move(int stacks[3], int count, int fut[3]);
void print_stacks(int stacks[3]);
void solve(int stacks[3], int count, int fut[3]);

// FUT = from using to
int main() {
	int stacks[3] = {height, 0, 0},
	    fut[3] = {0,1,2};
	print_stacks(stacks);
	solve(stacks,height,fut);
}

void move(int stacks[3], int count, int fut[3]) {
	int f = fut[0],
	    u = fut[1],
	    t = fut[2];
	if (count==1) {
		stacks[f] -= 1;
		stacks[t] += 1;
	} else if (!(count)) ;
	else {
		int ftu[3] = {f,t,u};
		move(stacks, count - 1, ftu);
	}
	print_stacks(stacks);
	printf("\n");
}

void print_stacks(int stacks[3]) {
	int i;
	for (i = 0; i < 3; i++) {
		int k, j = stacks[i];
		for (k = 0; k < j; k++) printf("-");
		printf("\n");
	}
	printf("Stack 1: %d\tStack 2: %d\tStack 3: %d\n", stacks[0], stacks[1], stacks[2]);
}

void solve(int stacks[3], int count, int fut[3]) {
	int f = fut[0],
	    u = fut[1],
	    t = fut[2];
	if (!count) ;
	else if (count == 1) move(stacks,count,fut);
	else {
		int ftu[3] = {f,t,u};
		solve(stacks,count-1,ftu);
		solve(stacks,1,fut);
		int uft[3] = {u,f,t};
		solve(stacks,count-1,uft);
	}
}
