/// General Command Line Manager
/// Defines:
///   command := cmdname [options] [value] [sub command]
///   cmdname := token
///   options := option*
///   option  := flag [value]
///   flag    := -token | --token
///   value   := token | "any"
/// Demo:
/// application_cmd (root command)
///   options
///   command
///     options
///     command
/// Sample:
///   ccc -h
///   ccc search "floor plan"
///   ccc run --quit-log 03s3 -t testcase1
///   ccc run --all
///   ccc run
///
use debug_ignore::DebugIgnore;

/// Command and Option's value Specification
#[derive(Debug, Copy, Clone)]
pub enum ValueSpec {
    Must,
    Maybe,
    None,
}

impl ValueSpec {
    /// make doc string for help makeing
    pub fn doc(&self) -> &str {
        match self {
            ValueSpec::Must => "VALUE",
            ValueSpec::Maybe => "[VALUE]",
            ValueSpec::None => "",
        }
    }
}

/// Command Specification
#[derive(Debug, Copy, Clone)]
pub struct CmdSpec<'a> {
    name: &'a str,    // command execute name
    title: &'a str,   // command short info in one sentence.
    usage: &'a str,   // command usage sample.
    desc: &'a str,    // command full description.
    vspec: ValueSpec, // command has value token or not
}

/// Command Execute Argumens
#[derive(Debug, Clone)]
pub struct CmdExeArgs<'a> {
    cmd: &'a CmdSpec<'a>,
    opns: Vec<&'a OpnSepc<'a>>,
    token: &'a str,
}

/// CLI Command's execute function
type FnCmder = fn(cmd_args: CmdExeArgs) -> Result<i32, String>;

/// Command Assembly
#[derive(Debug, Clone)]
pub struct Command {
    spec: CmdSpec<'static>,
    options: Vec<Option>,
    subcmds: Vec<Command>,
    exec_fn: DebugIgnore<FnCmder>,
}

/// Option Sepcification
#[derive(Debug, Clone, Copy)]
pub struct OpnSepc<'a> {
    short: &'a str, // option short flag
    long: &'a str,  // option long flag
    title: &'a str,
    vspec: ValueSpec, // option has value token or not
}

/// Option Assembly
#[derive(Debug, Clone)]
pub struct Option {
    spec: OpnSepc<'static>,
}

impl Option {
    /// Try match self Option with CLI arguments
    pub fn try_match(&self, arg: &str) -> bool {
        self.spec.short == arg || self.spec.long == arg
    }
}

impl Command {
    /// new root command, new sub-command with reg_subcmd
    pub fn new(sepc: CmdSpec<'static>, exec: FnCmder) -> Command {
        Command {
            spec: sepc,
            options: Vec::new(),
            subcmds: Vec::new(),
            exec_fn: exec.into(),
        }
    }

    /// Try match self command with CLI arguments
    pub fn try_match(&self, arg: &str) -> bool {
        self.spec.name == arg
    }

    /// new and register sub command
    pub fn reg_subcmd(&mut self, spec: CmdSpec<'static>, exec: FnCmder) -> &mut Command {
        self.subcmds.push(Command::new(spec, exec));
        self.subcmds.last_mut().unwrap()
    }

    /// new and registe option to self command
    pub fn reg_option(&mut self, spec: OpnSepc<'static>) -> &Option {
        self.options.push(Option { spec });
        self.options.last_mut().unwrap()
    }

    /// make command document
    pub fn make_doc(&self) -> Vec<String> {
        let mut doc: Vec<String> = Vec::new();

        // 1) command self document
        doc.push(format!("{:-^50}", "Brife Info"));
        doc.push(format!("{:>8} : {}", "Name", self.spec.name));
        doc.push(format!("{:>8} : {}", "Title", self.spec.title));
        doc.push(format!("{:>8} : {}", "Usage", self.spec.usage));
        doc.push(format!("{:>8} : {}", "Value", self.spec.vspec.doc()));
        if self.spec.desc.len() > 0 {
            doc.push(format!("{:>8} : {}", "Desc", self.spec.desc));
        }

        // 2) options brife document
        if self.options.len() > 0 {
            doc.push(format!("{:-^50}", "Options"));
            for option in &self.options {
                let flag = |x: &str, p: &str| {
                    if x.len() == 0 {
                        "".to_string()
                    } else {
                        format!("{}{}", p, x)
                    }
                };
                doc.push(format!(
                    "{:4} {:10} {:6} {}",
                    flag(option.spec.short, "-"),
                    flag(option.spec.long, "--"),
                    option.spec.vspec.doc(),
                    option.spec.title
                ));
            }
        }

        // 3) sub commands brife document
        if self.subcmds.len() > 0 {
            doc.push(format!("{:-^50}", "Sub Commands"));
            for subcmd in &self.subcmds {
                doc.push(format!(
                    "{:>10} {:6} {}",
                    subcmd.spec.name,
                    subcmd.spec.vspec.doc(),
                    subcmd.spec.title
                ));
            }
        }

        dbg!(doc.clone());
        doc
    }

    /// Parse command line and seek exec function
    pub fn parse(&self, args: Vec<String>) -> CmdExeArgs {
        todo!();
        // let mut cmd_args: CmdExeArgs;
        // cmd_args.cmd = &self.spec;
        // for arg in args {
        //     if arg.starts_with("-") {
        //         // try match options
        //         let token = arg.trim_start_matches("-");
        //         for opn in self.options {
        //             if opn.try_match(token) {
        //                 cmd_args.opns.push(&opn.spec);
        //                 continue;
        //             }
        //         }
        //     } else {
        //     }
        // }
        // cmd_args
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn default_cmder<'a>(cmd_args: CmdExeArgs) -> Result<i32, String> {
        dbg!(cmd_args);
        Ok(0)
    }

    fn build_commands() -> Command {
        let mut rtcmd = Command::new(
            CmdSpec {
                name: "ccc",
                title: "canada computer compenont",
                usage: "ccc -options sub-command",
                vspec: ValueSpec::Maybe,
                desc: "",
            },
            default_cmder,
        );
        rtcmd.reg_option(OpnSepc {
            short: "h",
            long: "help",
            title: "help information about application",
            vspec: ValueSpec::None,
        });
        rtcmd.reg_option(OpnSepc {
            short: "",
            long: "dir",
            title: "assign testcase base direction",
            vspec: ValueSpec::Must,
        });
        let runcmd = rtcmd.reg_subcmd(
            CmdSpec {
                name: "run",
                title: "run ccc question solvment",
                usage: "run 03s3 -t tc1",
                desc: "",
                vspec: ValueSpec::Maybe,
            },
            default_cmder,
        );
        runcmd.reg_option(OpnSepc {
            short: "t",
            long: "testcase",
            title: "testcase file name",
            vspec: ValueSpec::Must,
        });
        let searchcmd = rtcmd.reg_subcmd(
            CmdSpec {
                name: "search",
                title: "Search ccc question",
                usage: "ccc search 03s3",
                desc: "",
                vspec: ValueSpec::Must,
            },
            default_cmder,
        );
        searchcmd.reg_option(OpnSepc {
            short: "i",
            long: "letter-case",
            title: "Search by letter case sentitive",
            vspec: ValueSpec::None,
        });
        rtcmd
    }

    #[test]
    fn cmd_specification() {
        let rtcmd = build_commands();
        let doc = rtcmd.make_doc();
        assert_eq!(doc[0], "--------------------Brife Info--------------------");
        assert_eq!(doc[1], "    Name : ccc");
        assert_eq!(doc[2], "   Title : canada computer compenont");
        assert_eq!(doc[3], "   Usage : ccc -options sub-command");
        assert_eq!(doc[4], "   Value : [VALUE]");
        assert_eq!(doc[5], "---------------------Options----------------------");
        assert_eq!(
            doc[6],
            "-h   --help            help information about application"
        );
        assert_eq!(
            doc[7],
            "     --dir      VALUE  assign testcase base direction"
        );
        assert_eq!(doc[8], "-------------------Sub Commands-------------------");
        assert_eq!(doc[9], "       run [VALUE] run ccc question solvment");
        assert_eq!(doc[10], "    search VALUE  Search ccc question");

        let doc = rtcmd.subcmds[0].make_doc();
        assert_eq!(doc[0], "--------------------Brife Info--------------------");
        assert_eq!(doc[1], "    Name : run");
        assert_eq!(doc[2], "   Title : run ccc question solvment");
        assert_eq!(doc[3], "   Usage : run 03s3 -t tc1");
        assert_eq!(doc[4], "   Value : [VALUE]");
        assert_eq!(doc[5], "---------------------Options----------------------");
        assert_eq!(doc[6], "-t   --testcase VALUE  testcase file name");
    }

    #[test]
    fn cmd_exec() {
        // let rtcmd = build_commands();
        // let r = rtcmd.exec(Vec::new(), "".to_string());
        // assert!(r.is_err());
    }
}
