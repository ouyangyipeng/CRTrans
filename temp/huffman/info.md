# Info for huffman.c

## Description
A Huffman coding implementation that reads binary data from stdin, builds a Huffman tree based on byte frequencies, prints the generated Huffman codes for each byte present, and outputs the encoded bitstring as '0'/'1' characters.

## Samples
### Sample 1
Input:
````
hello
````
Output:
````
'e' : 110
'h' : 10
'l' : 0
'o' : 111

````
Return code: 0

### Sample 2
Input:
````
aaabbc
````
Output:
````
'a' : 0
'b' : 11
'c' : 10

````
Return code: 0

### Sample 3
Input:
````
AB
````
Output:
````
'A' : 0
'B' : 1

````
Return code: 0

### Sample 4
Input:
````
test
````
Output:
````
'e' : 10
's' : 11
't' : 0

````
Return code: 0

## Notes
The program consumes stdin twice (reopens /dev/stdin). For single-symbol input, code '0' is assigned. Output includes both codes and encoded bitstring.