#include <stdlib.h>
#include <stdio.h>
struct node;
struct tree;
struct node {
	struct node *l, *r, *b;
	int val;
};
struct tree {
	struct node *root;
};
struct node * node(int num);
struct tree tree(int num);
int rec_size(struct node * top);
void insert(struct node * top, int num);
int * to_array(struct tree);
int main() {
	return 0;
}
struct node * node(int num) {
	struct node * out;
	out = (struct node*) malloc(sizeof(struct node));
	if (out==NULL) return NULL;
	out->val = num;
	out->l = NULL;
	out->r = NULL;
	out->b = NULL;
}
struct tree tree(int num) {
	struct node *root;
	struct tree out;
	root = node(num);
	out = struct tree { root };
	return out;
}
int rec_size(struct node * top) {
	return (top == NULL)?0:(1+rec_size(top->l)+rec_size(top->r));
}
void insert(struct node * top, int num) {
	if (top->val == num) return;
	if (rec_size(top) == 1) {
		if (top->val > num) {
			root->l = node(num);
		} else {
			root->r = node(num);
		}
	} else {
		if (top->val > num && top->l) {
			insert(top->l, num);
		} else {
