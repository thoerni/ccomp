## ccomp
### A simplified solution to quickly compile and execute C files.
ccomp searches through your current working directory and automatically compiles all c files, using either `clang` or `gcc`,
and optionally runs the executable afterwards.

### Usage
ccomp is just a small wrapper for either gcc or clang. By default, ccomp uses `clang`, uses the arguments `--std=11 -Wall` and outputs to `c.out`.
- `--gcc` Changes the used compiler to gcc.
- `--out <file>` / `-o <file>` Changes the output file name.
- `--execute` / `--execute=arg1,arg2` / `--execute="arg1 arg2"` Runs the output file after compilation with the given arguments.
- `--overwrite` / `--overwrite=arg1,arg2` / `--overwrite="arg1 arg2"` Removes or overwrites the default arguments.

### Examples
- `ccomp` -> `clang file1.c file2.c <...> -o c.out --std=11 -Wall`
- `ccomp --gcc --overwrite` -> `gcc file1.c file2.c <...> -o c.out`
- `ccomp file10.c --overwrite="--std=10" --execute=z.txt -o a.out` -> `clang file10.c -o a.out --std=10`, `./a.out z.txt`

### Linux Setup
- [install ccomp](https://github.com/thoerni/ccomp/releases/tag/0.1.0)
- make sure the file is an executable `chmod +x ccomp`
- move the installed file into /usr/local/bin/ `mv ccomp /usr/local/bin/`
