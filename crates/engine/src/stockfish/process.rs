use std::io::{
    BufRead,
    BufReader,
    Write,
};

use std::process::{
    Child,
    ChildStdin,
    ChildStdout,
    Command,
    Stdio,
};

use crate::error::EngineError;

pub struct StockfishProcess {

    pub child: Child,

    pub stdin: ChildStdin,

    pub stdout:
        BufReader<ChildStdout>,
}

impl StockfishProcess {

    pub fn spawn(
        path: &str,
    ) -> Result<Self, EngineError> {

        let mut child =
            Command::new(path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

        let stdin =
            child
                .stdin
                .take()
                .ok_or_else(|| {
                    EngineError::ProcessExited
                })?;

        let stdout =
            child
                .stdout
                .take()
                .ok_or_else(|| {
                    EngineError::ProcessExited
                })?;

        Ok(Self {

            child,

            stdin,

            stdout:
                BufReader::new(stdout),
        })
    }

    pub fn write_line(
        &mut self,
        line: &str,
    ) -> Result<(), EngineError> {

        writeln!(
            self.stdin,
            "{}",
            line,
        )?;

        self.stdin.flush()?;

        Ok(())
    }

    pub fn read_line(
        &mut self,
    ) -> Result<String, EngineError> {

        let mut line =
            String::new();

        self.stdout
            .read_line(&mut line)?;

        Ok(line)
    }
}