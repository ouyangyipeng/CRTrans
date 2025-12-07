// maze_search.c
// Usage: compile with: gcc -std=c11 -O2 maze_search.c -o maze_search
// Input format example:
// 5 7
// #######
// #S...E#
// #.###.#
// #.....#
// #######
// Output: length of shortest path and the path as list of coordinates (r,c) 0-based, or "NO PATH".

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct { int r, c; } P;
int dr[4] = {-1,1,0,0}, dc[4] = {0,0,-1,1};

int main(void){
    int R,C;
    if (scanf("%d %d%*c", &R, &C)!=2) return 1;
    char **g = malloc(R * sizeof(char*));
    for(int i=0;i<R;i++){
        g[i] = malloc(C+1);
        if(!fgets(g[i], C+2, stdin)){ fprintf(stderr,"read fail\n"); return 2; }
        if((int)strlen(g[i])>0 && g[i][C]=='\n') g[i][C]=0;
    }
    P s={-1,-1}, e={-1,-1};
    for(int i=0;i<R;i++) for(int j=0;j<C;j++){
        if(g[i][j]=='S') s=(P){i,j};
        if(g[i][j]=='E') e=(P){i,j};
    }
    if(s.r<0||e.r<0){ printf("NO START/END\n"); return 0; }

    int **vis = malloc(R*sizeof(int*));
    P **from = malloc(R*sizeof(P*));
    for(int i=0;i<R;i++){
        vis[i]= calloc(C, sizeof(int));
        from[i]= malloc(C*sizeof(P));
    }
    P *q = malloc(R*C*sizeof(P));
    int qs=0, qe=0;
    q[qe++] = s; vis[s.r][s.c]=1; from[s.r][s.c] = (P){-1,-1};
    int found=0;
    while(qs<qe){
        P cur = q[qs++];
        if(cur.r==e.r && cur.c==e.c){ found=1; break; }
        for(int k=0;k<4;k++){
            int nr = cur.r + dr[k], nc = cur.c + dc[k];
            if(nr<0||nr>=R||nc<0||nc>=C) continue;
            if(vis[nr][nc]) continue;
            if(g[nr][nc]=='#') continue;
            vis[nr][nc]= vis[cur.r][cur.c] + 1;
            from[nr][nc] = cur;
            q[qe++] = (P){nr,nc};
        }
    }
    if(!found){ printf("NO PATH\n"); return 0; }
    // reconstruct
    P cur = e; int len = vis[e.r][e.c]-1;
    P *path = malloc((len+1)*sizeof(P));
    int idx=0;
    while(!(cur.r== -1 && cur.c==-1)){
        path[idx++]=cur;
        cur = from[cur.r][cur.c];
    }
    printf("%d\n", idx-1);
    for(int i=idx-1;i>=0;i--){
        printf("%d %d\n", path[i].r, path[i].c);
    }
    return 0;
}
