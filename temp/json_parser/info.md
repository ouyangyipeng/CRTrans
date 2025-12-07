# Info for json_parser.c

## Description
A simple JSON parser that reads JSON from stdin and pretty-prints it with indentation, handling objects, arrays, strings, numbers, and literals (true/false/null).

## Samples
### Sample 1
Input:
````
{"key": "value"}
````
Output:
````
null

````
Return code: 0

### Sample 2
Input:
````
[1, 2, 3]
````
Output:
````
null

````
Return code: 0

### Sample 3
Input:
````
true
````
Output:
````
null

````
Return code: 0

### Sample 4
Input:
````
{"a": [1, null, false]}
````
Output:
````
null

````
Return code: 0

## Notes
Input must be valid JSON; the program does minimal error handling. It prints the formatted JSON to stdout.