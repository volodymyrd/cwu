use indicatif::ProgressBar;
use std::time::Duration;

pub(crate) struct Progress {
    progress_bar: ProgressBar,
}
impl Progress {
    pub fn with_spinner(message: &str) -> Self {
        let progress_bar = get_spinner(message);
        Self { progress_bar }
    }

    pub fn finish(&self) {
        self.progress_bar.finish_using_style()
    }
}

fn get_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        indicatif::ProgressStyle::with_template("{spinner:.cyan} {wide_msg}")
            .expect("to be a good template")
            .tick_strings(&[
                "●     ",
                "●●    ",
                "●●●   ",
                "●●●●  ",
                "●●●●● ",
                "●●●●●●",
                "●●●●●●",
            ]),
    );
    pb.enable_steady_tick(Duration::from_millis(150));
    pb.set_message(message.to_owned());
    pb
}
