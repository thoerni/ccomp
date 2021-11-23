use run_script;

pub const DEFAULT_OUT: &str = "c.out";
pub const DEFAULT_ARGS: &str = "--std=c11 -Wall";


/// Returns all filenames in the current working
/// directory ending with '.c'.
fn get_c_files() -> Vec<String> {

    let path = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return vec![]
    };

    let files = match std::fs::read_dir(path) {
        Ok(files) => files,
        Err(_) => return vec![]
    };

    files
        .into_iter()
        .filter_map(|dir| {
            let file_name = match dir {
                Ok(entry) => entry.file_name().to_str()
                    .unwrap_or_default().to_owned(),
                Err(_) => return None
            };

            if file_name.ends_with(".c") {
                return Some(file_name)
            }

            return None
        })
        .collect()
}


/// Enum of available compilers
pub enum Compiler {
    Gcc,
    Clang
}

impl Compiler {
    pub fn to_string(&self) -> &'static str {
        match self {
            &Self::Gcc => "gcc",
            &Self::Clang => "clang"
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::Clang
    }
}


/// Stores all important command line arguments in 
/// order to create the corresponding compile 
/// and execute commmands
pub struct Command {
    out: String,
    compiler: Compiler,
    execute: Option<Vec<String>>,
    files: Vec<String>,
    compiler_args: Vec<String>,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            out: DEFAULT_OUT.to_owned(),
            compiler: Compiler::default(),
            execute: None,
            files: get_c_files(),
            compiler_args: vec![DEFAULT_ARGS.to_owned()]
        }
    }
}

impl Command {

    pub fn set_output_file(
        &mut self, 
        out: String
    ) -> &mut Self {
        self.out = out;
        self
    }

    pub fn set_compiler(
        &mut self,
        compiler: Compiler,
    ) -> &mut Self {
        self.compiler = compiler;
        self
    }

    pub fn set_executer(
        &mut self,
        executer: Option<Vec<String>>
    ) -> &mut Self {
        self.execute = executer;
        self
    }

    pub fn set_files(
        &mut self,
        files: Vec<String>
    ) -> &mut Self {
        self.files = files;
        self
    }

    pub fn set_compiler_args(
        &mut self,
        compiler_args: Vec<String>
    ) -> &mut Self {
        self.compiler_args = compiler_args;
        self
    }

    /// Returns whether the output file 
    /// should be executed
    pub fn should_execute(&self) -> bool {
        self.execute.is_some()
    }

    /// Returns the command that should
    /// compile all given files
    pub fn compile_command(&self) -> String {
        format!(
            "{compiler} {files} -o {out} {args}",
            compiler = self.compiler.to_string(),
            files = self.files.join(" "),
            out = self.out,
            args = self.compiler_args.join(" ")
        )
    }

    /// Returns the command that should
    /// run the executable file
    pub fn execute_command(&self) -> String {
        format!(
            "./{out} {execute}",
            out = self.out,
            execute = self.execute.as_ref().unwrap_or(&vec![]).join(" ")
        )
    }

    /// Compiles using the corresponding
    /// command
    pub fn compile(&self) {

        let process = run_script::run(
            &self.compile_command(),
            &vec![],
            &run_script::ScriptOptions::new(),
        );

        let (code, _, error) = match process {
            Ok(output) => output,
            Err(_) => {
                eprintln!("Failed executing command: {}", self.compile_command());
                std::process::exit(1);
            }
        };

        if code != 0 {
            eprintln!("Error while compiling: {}", self.compile_command());
            eprintln!("{}", std::str::from_utf8(error.as_bytes()).unwrap_or(""));
        }
    }

    /// Executes the compiled file
    /// using the corresponding command
    pub fn execute(&self) {

        // Print output directly to stdout instead
        // of capturing it
        let mut options = run_script::ScriptOptions::new();
        options.output_redirection = run_script::IoOptions::Inherit;

        let _process = run_script::run(
            &self.execute_command(), 
            &vec![],
            &options,
        );
        
    }
}
