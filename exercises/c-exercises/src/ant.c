#include <stdio.h>
#include <time.h>

int size = 16;

char marked[3] = {' ', ' ', '\0'},
     clean[3] = {'@', '@', '\0'};

void mark(int grid[size][size], int pos[2]);
void move(int pos[2], int * direction);
int get(int grid[size][size], int pos[2]);
void update(int grid[size][size], int pos[2], int * direction);
void print_grid(int grid[size][size]);

int main() {
	int pos[2] = {size>>1, size>>1},
	    direction = 0, 
	    grid[size][size];
	int i, j;
	for (i = 0; i < size; i++) for (j = 0; j < size; j++)
		grid[i][j] = 0;
	for (;;) {
		update(grid,pos,&direction);
		clock_t start_time = clock();
		print_grid(grid);
		while(clock() < start_time + 128) ;
	}
}

void mark(int grid[size][size], int pos[2]) {
	grid[pos[0]][pos[1]] += 1;
	grid[pos[0]][pos[1]] %= 2;
}

void move(int pos[2], int * direction) {
	switch(*direction) {
		case 0:
			pos[0] = (pos[0] + (size - 1)) % size;
			break;
		case 1:
			pos[1] = (pos[1] + (size + 1)) % size;
			break;
		case 2:
			pos[0] = (pos[0] + (size + 1)) % size;
			break;
		case 3:
			pos[1] = (pos[1] + (size - 1)) % size;
			break;
		default:
			*direction = (*direction + 4) % 4;
			move(pos,direction);
			break;
	}
}

int get(int grid[size][size], int pos[2]) {
	return grid[pos[0]][pos[1]];
}

void update(int grid[size][size], int pos[2], int * direction) {
	mark(grid,pos);
	move(pos,direction);
	*direction+=get(grid,pos)?1:-1;
}

void print_grid(int grid[size][size]) {
	int i, j, pos[2];
	for (i = 0; i < size; i++) {
		for (j = 0; j < size; j++) {
			pos[0] = i;
			pos[1] = j;
			printf("%s", get(grid,pos)?marked:clean);
		}
		printf("\n");
	}
	for (i = 0; i < 64 - size; i++) printf("\n");
}
