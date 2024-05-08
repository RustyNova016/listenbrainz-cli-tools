pub mod playlist;
use color_eyre::owo_colors::OwoColorize;
use derive_builder::Builder;
use indicatif::ProgressBar;
use listenbrainz::raw::response::{UserListensListen, UserListensResponse};
use listenbrainz::raw::Client;
use once_cell::sync::Lazy;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

pub mod cli_paging;
pub mod extensions;
pub mod tokio;
pub mod traits;

#[derive(Clone, Debug, PartialEq, Eq, Builder)]
#[allow(missing_docs)]
/// Reader for the the User Listens endpoint
pub struct ListenAPIPaginator {
    #[builder(setter(into, strip_option))]
    /// The name of the target user
    user_name: String,

    #[builder(setter(into, strip_option), default)]
    /// The UNIX timestamp of the earliest listen to retreive
    min_ts: Option<i64>,

    #[builder(setter(into, strip_option), default)]
    /// The UNIX timestamp of the latest listen to retreive
    max_ts: Option<i64>,

    #[builder(setter(into, strip_option), default = "Some(999)")]
    /// The number of listen to retreive from the API.
    count: Option<u64>,

    #[builder(setter(into, strip_option), default)]
    time_range: Option<u64>,
}

impl ListenAPIPaginator {
    /// Update [`Self::max_ts`] for the latest listen in the response
    fn update_max_ts(&mut self, responce: &UserListensResponse) {
        self.max_ts = responce
            .payload
            .listens
            .iter()
            .min_by_key(|listen| listen.listened_at)
            .map(|latest_listen| latest_listen.listened_at);
    }

    /// Retreive the next page of listens
    pub fn next(&mut self, client: &Client) -> Result<UserListensResponse, listenbrainz::Error> {
        let response =
            client.user_listens(&self.user_name, self.min_ts, self.max_ts, self.count)?;
        self.update_max_ts(&response);
        Ok(response)
    }

    pub fn into_reader(self) -> ListenAPIReader {
        ListenAPIReader::new(self)
    }
}

pub struct ListenAPIReader {
    paginator: ListenAPIPaginator,
    page: Vec<UserListensListen>,
}

impl ListenAPIReader {
    pub fn new(paginator: ListenAPIPaginator) -> Self {
        Self {
            page: Vec::new(),
            paginator,
        }
    }
}

impl Iterator for ListenAPIReader {
    type Item = UserListensListen;

    fn next(&mut self) -> Option<Self::Item> {
        if self.page.is_empty() {
            let client = Client::new();
            let page = self.paginator.next(&client).unwrap();
            self.page.extend(page.payload.listens);
        }

        self.page.pop()
    }
}

pub(crate) static STATIC_LOGGER: Lazy<Arc<Mutex<Logger>>> =
    Lazy::new(|| Arc::new(Mutex::new(Logger::new())));

pub trait OverridePrint {
    fn override_print<I: AsRef<str>>(&self, msg: I);
}

pub struct Logger {
    print_override: Option<ProgressBar>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            print_override: None,
        }
    }

    pub fn set_override(&mut self, pg: ProgressBar) {
        self.print_override = Some(pg);
    }

    pub fn clear_overide(&mut self) {
        self.print_override = None;
    }

    fn print<T: Display + std::convert::AsRef<str>>(&self, string: T) {
        if let Some(overide) = &self.print_override {
            overide.println(string);
        } else {
            println!("{string}");
        }
    }

    pub fn println_cli<T: Display>(&self, string: T) {
        self.print(format!("{} {}", "[CLI Tools]".green(), string));
    }

    pub fn println_lis<T: Display>(&self, string: T) {
        self.print(format!("{} {}", "[Listenbrainz]".blue(), string));
    }

    pub fn println_mus<T: Display>(&self, string: T) {
        self.print(format!("{} {}", "[MusicBrainz]".bright_magenta(), string));
    }

    pub fn set_global_overide(pg: ProgressBar) {
        let static_clone = STATIC_LOGGER.clone();
        let mut logger = static_clone.lock().unwrap();
        logger.set_override(pg);
    }

    pub fn clear_global_overide() {
        let static_clone = STATIC_LOGGER.clone();
        let mut logger = static_clone.lock().unwrap();
        logger.clear_overide();
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

pub fn println_cli<T: Display>(string: T) {
    let static_clone = STATIC_LOGGER.clone();
    let logger = static_clone.lock().unwrap();
    logger.println_cli(string);
}

pub fn println_lis<T: Display>(string: T) {
    let static_clone = STATIC_LOGGER.clone();
    let logger = static_clone.lock().unwrap();
    logger.println_lis(string);
}

pub fn println_mus<T: Display>(string: T) {
    let static_clone = STATIC_LOGGER.clone();
    let logger = static_clone.lock().unwrap();
    logger.println_mus(string);
}
