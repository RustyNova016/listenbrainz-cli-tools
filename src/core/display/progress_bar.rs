use std::ops::Deref;
use std::time::Duration;

use futures::Stream;
use indicatif::{ProgressBar, ProgressStyle};

use crate::utils::logger::Logger;

pub struct ProgressBarCli {
    pg: ProgressBar,
}

impl ProgressBarCli {
    pub fn new<N: Into<u64>>(len: N, name: Option<&str>) -> Self {
        let mut progress_bar = ProgressBar::new(len.into());

        let mut style_string = "{wide_bar} {pos}/{len} | {eta_precise}".to_string();

        if let Some(name) = name {
            let name_style = format!("[{name}]");

            style_string = format!("{name_style} {style_string}");
        }

        progress_bar = progress_bar.with_style(
            ProgressStyle::with_template(&style_string).expect("Couldn't convert template schema"),
        );

        progress_bar.enable_steady_tick(Duration::from_secs(1));

        Logger::add_global_pg(progress_bar.clone());

        Self { pg: progress_bar }
    }

    pub fn wrap_stream<S: Stream + Unpin>(
        stream: S,
        name: Option<&str>,
    ) -> indicatif::ProgressBarIter<S> {
        let pg = Self::new(0_u64, name);

        pg.wrap_stream(stream)
    }
}

impl Deref for ProgressBarCli {
    type Target = ProgressBar;
    fn deref(&self) -> &Self::Target {
        &self.pg
    }
}

impl Drop for ProgressBarCli {
    fn drop(&mut self) {
        Logger::remove_global_pg(self.pg.clone());
    }
}
