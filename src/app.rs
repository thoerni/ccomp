use clap::{Arg, App};
use crate::command::{Compiler, DEFAULT_ARGS, DEFAULT_OUT};

pub fn app<'a, 'b>(name: &str, help: &'b Help) -> App<'a, 'b> {
    App::new(name)
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
            .value_name("args")
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
        .arg(Arg::with_name("overwrite")
            .long("overwrite")
            .value_name("args")
            .takes_value(true)
            .use_delimiter(true)
            .min_values(0)
            .help(&help.overwrite)
        )
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
    overwrite: String,
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

            overwrite: format!("Overwrites the default compiler arguments. \
                DO NOT USE to overwrite output file name, use the \
                integrated argument instead. Default is {}", DEFAULT_ARGS)
        }
    }
}
