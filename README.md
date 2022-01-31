## ccomp
### A simplified solution to quickly compile and execute C files.
ccomp searches through your current working directory and automatically compiles all c files, using either `clang` or `gcc`,
and optionally runs the executable afterwards.

### Usage
ccomp is just a small wrapper for either gcc or clang. By default, ccomp uses `clang`, uses the arguments `--std=c11 -Wall -Werror -g` and outputs to `c.out`.
- `--gcc` Changes the used compiler to gcc.
- `--out <file>` / `-o <file>` Changes the output file name.
- `--execute <args>`, `-x <args>` Runs the output file after compilation with the given arguments.
- `--overwrite <compiler args>`, `-w <compiler args>` Removes or overwrites the default arguments.
- `--valgrind <valgrind args>`, `-v <valgrind args>` Uses valgrind to execute the binary.
- `-- <files>` Files to compile.

### Examples
- `ccomp` -> `clang file1.c file2.c <...> -o c.out --std=c11 -Wall -Werror -g`
- `ccomp --gcc --overwrite` -> `gcc file1.c file2.c <...> -o c.out`
- `ccomp file10.c -w="--std=c17" -x z.txt -o a.out -v` -> `clang file10.c -o a.out --std=c17`, `valgrind ./a.out z.txt`

### Linux Setup
- [install ccomp](https://github.com/thoerni/ccomp/releases/tag/0.1.1) (`wget https://github.com/thoerni/ccomp/releases/download/0.1.1/ccomp` to download it via the terminal)
- make sure the file is an executable (`chmod +x ccomp`)
- move the installed file into /usr/local/bin/ so you can call it system-wide (`mv ccomp /usr/local/bin/`)
