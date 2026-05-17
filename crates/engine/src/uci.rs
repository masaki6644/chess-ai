use std::{
    io::{
        BufRead,
        BufReader,
        Write,
    },
    process::{
        Child,
        ChildStdin,
        ChildStdout,
        Command,
        Stdio,
    },
};

pub struct UciEngine {

    _child: Child,

    stdin: ChildStdin,

    stdout: BufReader<ChildStdout>,
}

impl UciEngine {

    pub fn new(
        path: &str,
    ) -> Self {

        let mut child =
            Command::new(path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect(
                    "failed to start engine"
                );

        let stdin =
            child
                .stdin
                .take()
                .expect("stdin missing");

        let stdout =
            child
                .stdout
                .take()
                .expect("stdout missing");

        Self {
            _child: child,
            stdin,
            stdout: BufReader::new(stdout),
        }
    }

    pub fn send(
        &mut self,
        cmd: &str,
    ) {

        writeln!(
            self.stdin,
            "{}",
            cmd,
        )
        .expect("uci write failed");

        self.stdin
            .flush()
            .expect("flush failed");
    }

    pub fn read_line(
        &mut self,
    ) -> String {

        let mut line =
            String::new();

        self.stdout
            .read_line(&mut line)
            .expect(
                "uci read failed"
            );

        line
    }

    pub fn read_until(
        &mut self,
        token: &str,
    ) -> Vec<String> {

        let mut lines =
            Vec::new();

        loop {

            let line =
                self.read_line();

            let done =
                line.contains(token);

            lines.push(line);

            if done {
                break;
            }
        }

        lines
    }
}