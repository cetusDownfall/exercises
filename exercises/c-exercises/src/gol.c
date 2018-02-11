#include <stdio.h>
#include <time.h>
// Note to self: take a look at ncurses, also look at graphics libraries for c.
int size = 32;
int area = 1024;
char alive = ' ', dead = '@';

void row(char world[area], char r[size + 1], int ind);
void print_universe(char world[area]);
int mod_ind(int row, int col);
int get_neighbors(char world[area], int row, int col);
char get_next(char world[area], int row, int col);
void get_next_world(char world[area], char next_world[area]);
void mut_get_next_w(char world[area]);

int main() {
	char world[area];
	int i;
	for (i = 0; i < area; i++) world[i] = i^2
					    &&i^35
					    &&i^67
				   	    &&i^66
					    &&i^65?
					    	dead
						:alive;
	for (;;printf("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"
		       "\n\n\n\n\n\n\n")) {
		clock_t start_time = clock();
		mut_get_next_w(world);
	       	print_universe(world);
		while(clock() < start_time + 200) ;
	}
	return 0;
}

void row(char world[area], char r[size], int ind) {
	int i, n = ind * size;
	for (i = 0; i < size; i++) r[i] = world[n+i];
}

void print_universe(char world[area]) {
	int i;
	char r[size];
	for (i = 0; i < size; i++) {
		row(world,r,i);
		int n;
		for (n = 0; n < size; n++) printf("%c%c", r[n], r[n]);
		printf("\n");
	}
}

int mod_ind(int row, int col) {
	row = ((row + size) % size);
	col = ((col + size) % size);
	return (row * size) + col;
}

int get_neighbors(char world[area], int row, int col) {
	int i, j, c = 0;
	for (i = 0; i < 3; i++) {
		for (j = 0; j < 3; j++) {
			if ((!(i==1&&j==1)) 
			   && alive == world[mod_ind(row + (i-1), col + (j-1))]) 
				c++;
			if (c == 4) break;
		}
		if (c == 4) break;
	}
	return c;
}

char get_next(char world[area], int row, int col) {
	int ct = get_neighbors(world,row,col);
	return ct < 2 || ct == 4 ? 
	       dead 
	       : ct == 2 ? 
	         world[mod_ind(row, col)] 
		 : alive;
}

void get_next_world(char world[area], char next_world[area]) {
	int r, c;
	for (r = 0; r < size; r++) for (c = 0; c < size; c++)
		next_world[mod_ind(r,c)] = get_next(world,r,c);
}
void mut_get_next_w(char world[area]) {
	char next_world[area];
	get_next_world(world, next_world);
	int i;
	for (i = 0; i < area; i++) world[i] = next_world[i];
}
