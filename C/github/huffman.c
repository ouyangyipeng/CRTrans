// huffman.c
// Simple Huffman coding demo: reads from stdin, counts byte frequencies, builds Huffman tree,
// prints codes for bytes present and a simple encoded bitstring (as '0'/'1' chars) for the input.
// Compile: gcc -std=c11 -O2 huffman.c -o huffman
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXSYM 256
typedef struct Node {
    int sym;
    unsigned long freq;
    struct Node *l, *r;
} Node;

typedef struct {
    Node **a; int n;
} Heap;

Heap *heap_new(int cap){ Heap *h=malloc(sizeof(Heap)); h->a=malloc(cap*sizeof(Node*)); h->n=0; return h; }
void heap_push(Heap *h, Node *x){ int i=h->n++; h->a[i]=x; while(i){ int p=(i-1)/2; if(h->a[p]->freq <= h->a[i]->freq) break; Node *t=h->a[p]; h->a[p]=h->a[i]; h->a[i]=t; i=p; } }
Node *heap_pop(Heap *h){ if(h->n==0) return NULL; Node *ret=h->a[0]; h->a[0]=h->a[--h->n]; int i=0; while(1){ int l=2*i+1, r=2*i+2, smallest=i; if(l<h->n && h->a[l]->freq < h->a[smallest]->freq) smallest=l; if(r<h->n && h->a[r]->freq < h->a[smallest]->freq) smallest=r; if(smallest==i) break; Node *t=h->a[i]; h->a[i]=h->a[smallest]; h->a[smallest]=t; i=smallest; } return ret; }

Node *node_new(int sym, unsigned long freq){ Node *n=malloc(sizeof(Node)); n->sym=sym; n->freq=freq; n->l=n->r=NULL; return n; }

char *codes[MAXSYM];
void gen_codes(Node *root, char *buf, int depth){
    if(!root) return;
    if(root->l==NULL && root->r==NULL){
        buf[depth]=0;
        codes[root->sym]=strdup(buf);
        return;
    }
    if(root->l){ buf[depth]='0'; gen_codes(root->l, buf, depth+1); }
    if(root->r){ buf[depth]='1'; gen_codes(root->r, buf, depth+1); }
}

int main(void){
    unsigned long freq[MAXSYM]={0};
    int c;
    unsigned long total=0;
    // read stdin binary
    while((c=fgetc(stdin))!=EOF){ freq[(unsigned char)c]++; total++; }
    if(total==0){ fprintf(stderr,"no input\n"); return 0; }
    Heap *h=heap_new(MAXSYM);
    for(int i=0;i<MAXSYM;i++) if(freq[i]) heap_push(h, node_new(i, freq[i]));
    if(h->n==1){ // single symbol special case
        Node *leaf=heap_pop(h);
        codes[leaf->sym]=strdup("0");
    } else {
        while(h->n>1){
            Node *a=heap_pop(h), *b=heap_pop(h);
            Node *p=node_new(-1,a->freq+b->freq);
            p->l=a; p->r=b;
            heap_push(h,p);
        }
        Node *root = heap_pop(h);
        char buf[512];
        gen_codes(root, buf, 0);
    }
    // print codes present
    for(int i=0;i<MAXSYM;i++) if(codes[i]){
        if(i>=32 && i<127) printf("'%c' : %s\n", (char)i, codes[i]);
        else printf("%02x : %s\n", i, codes[i]);
    }
    // encode by re-reading stdin by reopening /dev/stdin (since we consumed it)
    FILE *f = fopen("/proc/self/fd/0", "r");
    if(!f) { fprintf(stderr,"cannot reopen stdin for encoding; codes created\n"); return 0; }
    while((c=fgetc(f))!=EOF){
        fputs(codes[(unsigned char)c], stdout);
    }
    fclose(f);
    return 0;
}
