## ccomp
### A simplified solution to quickly compile and execute C files.
ccomp searches through your current working directory and automatically compiles all c files, using either `clang` or `gcc`,
and optionally runs the executable afterwards.

### Usage
```
USAGE:
    ccomp [FLAGS] [OPTIONS] [INPUT]...

FLAGS:
        --gcc        Changes the C compiler used to gcc. clang is used by default.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --args <args>            Overwrites the default compiler arguments. DO NOT USE to overwrite output file name,
                                 use the integrated argument instead. Default is '--std=c11 -Wall -Werror -g'.
    -x, --execute <with args>    Runs the executable after compiling with all given arguments.
    -o, --out <file>             Changes the name of the output file. Default is c.out.
    -v, --valgrind <valgrind>    Specifies whether the code should be executed using Valgrind. If this flag is enabled,
                                 the execute flag won't execute, but its arguments will be used for Valgrind.

ARGS:
    <INPUT>...    Sets the files to be compiled. If no file is specified, all files ending with '.c' will be
                  compiled.
```
      
### Examples
```
~$ ccomp
-> Compiling: clang file1.c file2.c <...> -o c.out --std=c11 -Wall -Werror -g

~$ ccomp --gcc -a
-> Compiling: gcc file1.c file2.c <...> -o c.out

~$ ccomp afile.c -w="--std=c17" -x z.txt -o a.out -v
-> Compiling: clang afile.c -o a.out --std=c17
-> Executing: valgrind ./a.out z.txt
```

### Linux Setup
- [install ccomp](https://github.com/thoerni/ccomp/releases/tag/0.1.1) (`wget https://github.com/thoerni/ccomp/releases/download/0.1.1/ccomp` to download it via the terminal)
- make sure the file is an executable (`chmod +x ccomp`)
- move the installed file into /usr/local/bin/ so you can call it system-wide (`mv ccomp /usr/local/bin/`)
