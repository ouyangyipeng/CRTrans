// tricky_void_container.c
/* Demonstrates void* container holding different typed pointers and runtime tag dispatch.
   Theoretical Rust mapping: use an enum like:
   enum Value { Int(i32), Double(f64), Ptr(Box<[u8]>) }
   Avoid void* in Rust; represent variant types explicitly with enums and owned containers.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int tag; // 1=int, 2=double, 3=string
    void *p;
} Holder;

int main(void){
    Holder h;
    h.tag = 1;
    int *a = malloc(sizeof(int));
    *a = 42;
    h.p = a;
    if(h.tag==1){
        int val = *(int*)(h.p);
        printf("int %d\n", val);
    }
    // replace with string
    free(a);
    h.tag = 3;
    char *s = malloc(16);
    strcpy(s, "hello");
    h.p = s;
    if(h.tag==3){
        printf("str %s\n", (char*)h.p);
    }
    free(h.p);
    return 0;
}
