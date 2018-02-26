#include <stdlib.h>

struct Node; 
struct Node {
	void* conts;
	Node* next;
}
struct CircBuffer {
	Node* head;
}
