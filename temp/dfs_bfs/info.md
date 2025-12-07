# Info for dfs_bfs.c

## Description
Interactive program to build and traverse an undirected graph using adjacency matrix, with options for DFS and BFS traversal.

## Samples
### Sample 1
Input:
````
1 0 1
1 0 2
1 1 3
3 0
0
````
Output:
````

 1=create edge 
 2= delete edge 
 3= bfs 
 4=dfs 
 enter choice 

 Enter two numbers to create edgeContinue? 
 1=yes 
 0=no
 1=create edge 
 2= delete edge 
 3= bfs 
 4=dfs 
 enter choice 
Invalid inputContinue? 
 1=yes 
 0=no
````
Return code: 0

### Sample 2
Input:
````
1 5 6
1 6 7
4 5
0
````
Output:
````

 1=create edge 
 2= delete edge 
 3= bfs 
 4=dfs 
 enter choice 

 Enter two numbers to create edgeContinue? 
 1=yes 
 0=no
 1=create edge 
 2= delete edge 
 3= bfs 
 4=dfs 
 enter choice 
Invalid inputContinue? 
 1=yes 
 0=no
````
Return code: 0

### Sample 3
Input:
````
1 2 3
2 2 3
3 2
0
````
Output:
````

 1=create edge 
 2= delete edge 
 3= bfs 
 4=dfs 
 enter choice 

 Enter two numbers to create edgeContinue? 
 1=yes 
 0=no
````
Return code: 0

### Sample 4
Input:
````
1 1 2
1 2 3
4 1
0
````
Output:
````

 1=create edge 
 2= delete edge 
 3= bfs 
 4=dfs 
 enter choice 

 Enter two numbers to create edgeContinue? 
 1=yes 
 0=no
 1=create edge 
 2= delete edge 
 3= bfs 
 4=dfs 
 enter choice 

enter two numbers to delete edgeContinue? 
 1=yes 
 0=no
 1=create edge 
 2= delete edge 
 3= bfs 
 4=dfs 
 enter choice 
Invalid inputContinue? 
 1=yes 
 0=no
````
Return code: 0

## Notes
Vertices are numbered 0-9. Input format: menu choice followed by required parameters. Use 0 to exit loop.