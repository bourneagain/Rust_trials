This is an example of Rust code being called from C.
Essentially we need the following:
1. Have rust code export extern function and use cbindgen to generate C headers. Build the rust library to generate the .a/.so target that we can link to C
2. Have C code import the rust header and during compilation ensure to link the .a 
ex: gcc main.c -o main -l:libtestadd.a -L. 
where,
 -l specifices the library we are linking and 
 -L specifies the path to the library

