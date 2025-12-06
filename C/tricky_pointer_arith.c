// tricky_pointer_arith.c
/* Pointer arithmetic, slices and sub-slices mapping:
   If C does `p + offset` and treats it as a subrange, in Rust prefer `&arr[offset..]` or split_at_mut.
   Avoid raw pointer math if possible; use safe slice APIs.
*/

#include <stdio.h>
#include <stdlib.h>

int sum_from(int *arr, int n, int start){
    int sum=0;
    int *p = arr + start;
    for(int i=start;i<n;i++) sum += p[i-start];
    return sum;
}

int main(void){
    int n=5;
    int *a = malloc(n * sizeof(int));
    for(int i=0;i<n;i++) a[i]=i+1;
    printf("%d\n", sum_from(a,n,2)); // 3+4+5 = 12
    free(a);
    return 0;
}
