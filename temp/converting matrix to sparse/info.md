# Info for converting matrix to sparse.c

## Description
Reads two matrices from user input, displays them, and converts each to sparse representation in triple form (row, column, value for non-zero elements).

## Samples
### Sample 1
Input:
````
2 2 1 0 0 2 2 2 3 0 0 4
````
Output:
````

Enter the row and coloumn size of the 1st matrix : Enter elements of 1st matrix row wise :
Enter next element : Enter next element : Enter next element : Enter next element : 
Enter the row and coloumn size of the 2nd matrix : Enter elements of 2nd matrix row wise :
Enter next element : Enter next element : Enter next element : Enter next element : Entered matrixes are
1  0  
0  2  

 and 
3  0  
0  4  

Converting the first matrix to triple form

Sparse form list of matrix in triple form is
2	2	2
0	0	1
1	1	2

Converting the second matrix to triple form

Sparse form list of matrix in triple form is
2	2	2
0	0	3
1	1	4

````
Return code: 0

### Sample 2
Input:
````
3 3 0 0 5 0 0 0 0 0 0 3 3 0 0 0 0 0 0 0 0 0
````
Output:
````

Enter the row and coloumn size of the 1st matrix : Enter elements of 1st matrix row wise :
Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : 
Enter the row and coloumn size of the 2nd matrix : Enter elements of 2nd matrix row wise :
Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Entered matrixes are
0  0  5  
0  0  0  
0  0  0  

 and 
0  0  0  
0  0  0  
0  0  0  

Converting the first matrix to triple form

Sparse form list of matrix in triple form is
3	3	1
0	2	5

Converting the second matrix to triple form

Sparse form list of matrix in triple form is
3	3	0

````
Return code: 0

### Sample 3
Input:
````
1 3 7 0 9 2 3 0 0 0 0 0 0
````
Output:
````

Enter the row and coloumn size of the 1st matrix : Enter elements of 1st matrix row wise :
Enter next element : Enter next element : Enter next element : 
Enter the row and coloumn size of the 2nd matrix : Enter elements of 2nd matrix row wise :
Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Enter next element : Entered matrixes are
7  0  9  

 and 
0  0  0  
0  0  0  

Converting the first matrix to triple form

Sparse form list of matrix in triple form is
1	3	2
0	0	7
0	2	9

Converting the second matrix to triple form

Sparse form list of matrix in triple form is
2	3	0

````
Return code: 0

### Sample 4
Input:
````
2 2 0 0 0 0 2 2 0 0 0 0
````
Output:
````

Enter the row and coloumn size of the 1st matrix : Enter elements of 1st matrix row wise :
Enter next element : Enter next element : Enter next element : Enter next element : 
Enter the row and coloumn size of the 2nd matrix : Enter elements of 2nd matrix row wise :
Enter next element : Enter next element : Enter next element : Enter next element : Entered matrixes are
0  0  
0  0  

 and 
0  0  
0  0  

Converting the first matrix to triple form

Sparse form list of matrix in triple form is
2	2	0

Converting the second matrix to triple form

Sparse form list of matrix in triple form is
2	2	0

````
Return code: 0

## Notes
First two integers per matrix are rows and columns. Inputs are small (max 10x10). Zero entries are omitted in sparse output. Triple form includes metadata row 0: total rows, columns, and non-zero count.