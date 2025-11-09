use std::io;
use std::io::Write;
use termimad::MadSkin;
use termimad::crossterm::style::Stylize;

pub(crate) struct Message {}

impl Message {
    pub fn error(error: &str) {
        let markdown_text = format!("**{}**", error);

        // 2. Use MadSkin to render the bold text (or any other Markdown)
        let skin = MadSkin::default();
        let styled_output = skin.inline(&markdown_text);

        // 3. Convert the styled output to a String and apply the Red color
        //    using the 'colored' crate, which uses direct ANSI codes.
        let final_output = styled_output.to_string().red().to_string();

        // 4. Print the final colored string to stderr
        eprintln!("{}", final_output);

        let _ = io::stderr().flush();
    }
}
