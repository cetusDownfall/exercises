#include <stdio.h>
#include <stdlib.h>
struct tree;
struct node;
struct node {
	struct node *l, *r, *b;
	int val;
};
struct tree {
	struct node *root;
};
struct node * node(int num);
void insert(int num, struct node * root);
int rec_size(struct node * root);
struct node * lowest_ptr(struct node * root);
struct node * next_ptr(struct node * root);
int * to_arr(struct node * root);
int main() {
	struct node *head;
	int i, *arr, s;
	head = node(5);
	for (i = 0; i < 10; i++) insert(i, head);
	arr = to_arr(head);
	for (i = 0; i < 10; i++) printf("%d ", *(arr + i));
	printf("\n");
	s = rec_size(head);
	return 0;
}
struct node * node(int num) {
	struct node *tree;
	tree = (struct node*) malloc(sizeof(struct node));
	if (tree == NULL) return NULL;
	tree->l = NULL;
	tree->r = NULL;
	tree->b = NULL;
	tree->val = num;
	return tree;
}
void insert(int num, struct node * root) {
	if (root->val > num) {
		if (root->l == NULL) {
			struct node * leaf;
			leaf = node(num);
			leaf->b = root;
			root->l = leaf;
		} else {
			insert(num,root->l);
		}
	} else if (root->val < num) {
		if (root->r == NULL) {
			struct node * leaf;
			leaf = node(num);
			leaf->b = root;
			root->r = leaf;
		} else {
			insert(num,root->r);
		}
	}
}
int rec_size(struct node * root) {
	return root?(1 + rec_size(root->l) + rec_size(root->r)):0;
}
struct node * lowest_ptr(struct node * root) {
	return root->l?lowest_ptr(root->l):root;
}
struct node * next_ptr(struct node * root) {
	if (root->r) 
		return lowest_ptr(root->r);
	if (root->b) {
		struct node *curr;
		for (curr = root->b; root->val > curr->val && curr->b; curr = curr->b);
		return curr;
	}
	return root;
}
int * to_arr(struct node * root) {
	int *out, i, s;
	struct node * curr;
	s = rec_size(root);
	out = (int *)malloc(s * sizeof(int));
	if (out == 0) return NULL;
	for (curr = lowest_ptr(root), i = 0; i < s; curr = next_ptr(curr), i++) *(out + i) = curr->val;
	return out;
}
