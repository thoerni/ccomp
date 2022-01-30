use clap::{Arg, App};
use crate::command::{Compiler, DEFAULT_ARGS, DEFAULT_OUT};

pub fn app<'a, 'b>(name: &str, help: &'b Help) -> App<'a, 'b> {
    App::new(name)
        .version("0.1.1")
        .author("Max Thorn <maxpaul.thorn@gmail.com>")
        .arg(Arg::with_name("INPUT")
            .multiple(true)
            .help(&help.input)
        )
        .arg(Arg::with_name("gcc")
            .long("gcc")
            .help(&help.gcc)
        )
        .arg(Arg::with_name("execute")
            .long("execute")
            .short("x")
            .value_name("with args")
            .takes_value(true)
            .use_delimiter(true)
            .min_values(0)
            .help(&help.execute)
        )
        .arg(Arg::with_name("out")
            .long("out")
            .short("o")
            .value_name("file")
            .takes_value(true)
            .help(&help.out)
        )
        .arg(Arg::with_name("args")
            .long("args")
            .short("a")
            .value_name("args")
            .takes_value(true)
            .use_delimiter(true)
            .min_values(0)
            .help(&help.args)
        )
        .arg(Arg::with_name("valgrind")
            .long("valgrind")
            .short("v")
            .value_name("valgrind")
            .help(&help.valgrind)
        )
        // .arg(Arg::with_name("run")
        //     .long("run")
        //     .short("r")
        //     .value_name("scripts")
        //     .takes_value(true)
        //     .use_delimiter(true)
        //     .help(&help.scripts)
        // )
}


// Due to clap only accepting &str, dynamic help messages
// (for eventually changing variables such as DEFAULT_ARGS)
// are only possilbe if created seperately and being passed
// to clap later on (or via third party packages using macros).
pub struct Help {
    input: String,
    gcc: String,
    execute: String,
    out: String,
    args: String,
    valgrind: String,
    // scripts: String,
}

impl Default for Help {
    fn default() -> Self {
        Self {
            input: "Sets the files to be compiled. \
                If no file is specified, all files ending \
                with '.c' will be compiled.".to_owned(),

            gcc: format!("Changes the C compiler used to gcc. \
                {} is used by default.", Compiler::default().to_string()),

            execute: "Runs the executable after compiling \
                with all given arguments.".to_owned(),

            out: format!("Changes the name of the output file. \
                Default is {}.", DEFAULT_OUT),

            args: format!("Overwrites the default compiler arguments. \
                DO NOT USE to overwrite output file name, use the \
                integrated argument instead. Default is '{}'.", DEFAULT_ARGS),
            
            valgrind: "Specifies whether the code should be executed using \
                Valgrind. If this flag is enabled, the execute flag won't execute, \
                but its arguments will be used for Valgrind.".to_owned(),
            
            // scripts: "Run one or multiple scripts from the .ccomp file. The \
            //     File format is as follows: \
            //         <script_name>: \
            //             Files: <files>, \
            //             <command>: <args>, \
            //             ... \
            //     if the command has no arguments or none \
            //     are specified the default values will be used.".to_owned()
        }
    }
}
