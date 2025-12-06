# Info for json_parser.c

## Description
A minimal JSON parser that reads JSON from stdin and pretty-prints it with indentation, handling objects, arrays, strings, numbers, true/false/null.

## Samples
### Sample 1
Input:
````
{"key": 42}
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
{"a": ["b", null]}
````
Output:
````
null

````
Return code: 0

## Notes
Input must be valid JSON; errors may cause truncated output. The parser does not validate thoroughly.