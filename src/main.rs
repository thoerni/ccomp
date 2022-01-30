mod app;
mod command;
mod out;

use command::Command;
use clap::{ArgMatches, Values};


fn main() {

    let help = app::Help::default();
    let matches = app::app("ccomp", &help).get_matches();
    let mut command = Command::default();
    set_arguments(&mut command, &matches);

    out::status(&format!(
        "Compiling: {}", 
        command.compile_command()
    ));
    command.compile();

    let exit_code = {
        if command.should_execute() {
            out::status(&format!(
                "Executing: {}", 
                command.execute_command()
            ));
            command.execute(command.execute_command())
        }

        else if command.use_valgrind() {
            out::status(&format!(
                "Executing: {}", 
                command.execute_with_valgrind()
            ));
            command.execute(command.execute_with_valgrind())
        }
    
        else { 0 }
    };

    std::process::exit(exit_code)
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

    if let Some(compiler_args) = matches.values_of("args") {
        command.set_compiler_args(values_to_vec(compiler_args));
    }

    if let Some(valgrind_args) = matches.values_of("valgrind") {
        command.set_valgrind(Some(values_to_vec(valgrind_args)));
    }

}