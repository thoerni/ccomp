mod app;
mod command;

use command::Command;
use clap::{ArgMatches, Values};


fn main() {

    let help = app::Help::default();
    let matches = app::app("ccomp", &help).get_matches();

    let mut command = Command::default();
    set_arguments(&mut command, &matches);

    println!("Matches: {:#?}", matches);

    println!("Compiling: {}", command.compile_command());
    command.compile();
    
    // std::process::Command::
}


fn values_to_vec(values: Values) -> Vec<String> {
    values
        .map(|f| f.to_owned())
        .collect()
}

fn set_arguments(command: &mut Command, matches: &ArgMatches) {

    if let Some(out) = matches.value_of("out") {
        command.set_output_file(out.to_owned());
    }

    if let 1 = matches.occurrences_of("gcc") {
        command.set_compiler(command::Compiler::Gcc);
    }

    if let Some(execute_args) = matches.values_of("execute") {
        command.set_executer(Some(values_to_vec(execute_args)));
    }

    if let Some(input_files) = matches.values_of("INPUT") {
        command.set_files(values_to_vec(input_files));
    }

    if let Some(compiler_args) = matches.values_of("overwrite") {
        command.set_compiler_args(values_to_vec(compiler_args));
    }

}