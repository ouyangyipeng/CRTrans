// json_parser.c - tiny JSON parser (supports objects, arrays, strings, numbers, true/false/null)
// Compile: gcc -std=c11 -O2 json_parser.c -o json_parser
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

const char *s; int pos=0;
void skipws(){ while(isspace((unsigned char)s[pos])) pos++; }

void print_indent(int d){ for(int i=0;i<d;i++) putchar(' '); }

void parse_value(int indent);

void parse_string(){
    putchar('"');
    pos++; // skip "
    while(s[pos] && s[pos]!='"'){
        if(s[pos]=='\\'){ putchar('\\'); pos++; if(s[pos]) putchar(s[pos]); pos++; continue; }
        putchar(s[pos]); pos++;
    }
    if(s[pos]=='"') { putchar('"'); pos++; }
}

void parse_number(){
    print_indent(0);
    int start=pos;
    if(s[pos]=='-') pos++;
    while(isdigit((unsigned char)s[pos])) pos++;
    if(s[pos]=='.'){ pos++; while(isdigit((unsigned char)s[pos])) pos++; }
    // just print number substring
    for(int i=start;i<pos;i++) putchar(s[i]);
}

void parse_array(int indent){
    printf("[\n");
    pos++; skipws();
    int first=1;
    while(s[pos] && s[pos]!=']'){
        if(!first) printf(",\n"); first=0;
        print_indent(indent+2);
        parse_value(indent+2);
        skipws();
        if(s[pos]==',') { pos++; skipws(); }
    }
    printf("\n"); print_indent(indent); printf("]");
    if(s[pos]==']') pos++;
}

void parse_object(int indent){
    printf("{\n");
    pos++; skipws();
    int first=1;
    while(s[pos] && s[pos]!='}'){
        if(!first) printf(",\n"); first=0;
        print_indent(indent+2);
        // key
        if(s[pos]=='\"'){ parse_string(); } else { printf("ERROR_KEY"); }
        skipws();
        if(s[pos]==':') pos++;
        skipws();
        printf(": ");
        parse_value(indent+2);
        skipws();
        if(s[pos]==','){ pos++; skipws(); }
    }
    printf("\n"); print_indent(indent); printf("}");
    if(s[pos]=='}') pos++;
}

void parse_value(int indent){
    skipws();
    if(s[pos]=='\"'){ parse_string(); }
    else if(s[pos]=='{'){ parse_object(indent); }
    else if(s[pos]=='['){ parse_array(indent); }
    else if(isdigit((unsigned char)s[pos]) || s[pos]=='-'){ parse_number(); }
    else if(strncmp(s+pos,"true",4)==0){ printf("true"); pos+=4; }
    else if(strncmp(s+pos,"false",5)==0){ printf("false"); pos+=5; }
    else if(strncmp(s+pos,"null",4)==0){ printf("null"); pos+=4; }
    else { printf("null"); pos++; }
}

int main(void){
    // read stdin fully
    fseek(stdin, 0, SEEK_END);
    long sz = ftell(stdin);
    fseek(stdin, 0, SEEK_SET);
    char *buf = malloc(sz+1);
    if(!buf) return 1;
    fread(buf,1,sz,stdin); buf[sz]=0;
    s = buf; pos=0;
    parse_value(0);
    putchar('\n');
    return 0;
}
