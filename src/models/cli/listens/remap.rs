use crate::core::entity_traits::mbid::is_cached_mbid::IsCachedMBID;
use crate::core::json_querry::ReadAsJSON;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::listenbrainz::messybrainz::msid::MSID;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::error::Error;
use crate::utils::println_cli;
use crate::utils::println_cli_err;
use crate::utils::println_cli_info;
use color_eyre::owo_colors::OwoColorize;
use inquire::Select;
use inquire::Text;
use json_to_table::json_to_table;
use std::collections::HashMap;
use strum::VariantArray;
use strum_macros::VariantArray;
use tabled::settings::Style;

#[derive(Debug)]
pub struct Remapper {
    filter: HashMap<String, serde_json::Value>,
    state: State,
    username: String,
    token: String,
}

impl Remapper {
    pub fn new(username: String, token: String) -> Self {
        Self {
            filter: HashMap::new(),
            state: State::Filter,
            token,
            username,
        }
    }

    pub async fn run(username: String, token: String) {
        let mut this = Self::new(username, token);

        while this.handle_state().await {}
    }

    async fn handle_state(&mut self) -> bool {
        match self.state {
            State::Filter => self.handle_filter_state(),
            State::Remap => self.handle_remap_state().await,
            State::Exit => return false,
        }

        true
    }

    fn handle_filter_state(&mut self) {
        if self.filter.is_empty() {
            self.create_new_filter_rule();
        } else {
            self.ask_for_filter_action();
        }
    }

    fn create_new_filter_rule(&mut self) {
        let field = ask_for_field_name();
        let value = ask_for_field_value();

        self.filter.insert(field, value);
    }

    fn ask_for_filter_action(&mut self) {
        println!();
        self.print_filter();
        println!();

        match Select::new("", FilterActions::VARIANTS.to_vec())
            .prompt()
            .unwrap()
        {
            FilterActions::Add => self.create_new_filter_rule(),
            FilterActions::Reset => self.reset_filter(),
            FilterActions::Exit => self.state = State::Exit,
            FilterActions::Remap => self.state = State::Remap,
        }
    }

    fn reset_filter(&mut self) {
        self.filter = HashMap::new();
    }

    fn print_filter(&self) {
        println_cli("Here is the current filter:");
        for (field, value) in &self.filter {
            println!("    - \"{field}\": {value}");
        }
    }

    async fn handle_remap_state(&mut self) {
        let listens = UserListens::get_user_with_refresh(&self.username)
            .await
            .expect("Couldn't fetch the new listens");

        let mut remapped_msids = Vec::new();

        for listen in listens.get_listens().clone().into_iter() {
            // Already remapped? Skip
            if remapped_msids.contains(listen.get_messybrain_data().msid()) {
                continue;
            }

            // Does it fit the filter?
            if !self.listen_fit_filter(listen.as_ref()) {
                continue;
            }

            remapped_msids.push(listen.get_messybrain_data().msid().clone());

            let messybrain_json = serde_json::to_value(listen.original_data().clone()).unwrap();

            println!();
            println!("{}", json_to_table(&messybrain_json).with(Style::modern()));
            println!();

            if !self.ask_remap_action(listen.as_ref()).await {
                return;
            }
        }

        if remapped_msids.is_empty() {
            println_cli("Couldn't find any listen matching the filter. Maybe try editing it?");
        } else {
            println!();
            println_cli("All listens have been remapped sucessfully!");
        }

        self.state = State::Filter;
    }

    async fn ask_remap_action(&mut self, listen: &Listen) -> bool {
        match Select::new(
            "What do you want to do with this listen data?",
            ListenActions::VARIANTS.to_vec(),
        )
        .prompt()
        .unwrap()
        {
            ListenActions::Exit => {
                self.state = State::Exit;
                false
            }
            ListenActions::ChangeFilter => {
                self.state = State::Filter;
                false
            }
            ListenActions::Remap => {
                self.do_remap(listen.get_msid(), ask_for_mbid().await).await;
                true
            }
            ListenActions::Skip => true,
        }
    }

    async fn do_remap(&self, msid: MSID, mbid: RecordingMBID) {
        msid.submit_mapping(mbid, &self.token).await.unwrap();
        println_cli("Remapped MSID");
    }

    fn listen_fit_filter(&self, listen: &Listen) -> bool {
        let listen = listen.clone();

        for (field, value) in &self.filter {
            let listen_value = listen
                .original_data()
                .clone()
                .get_field(field)
                .expect("Couldn't convert the listen back to JSON.");
            if &listen_value != value {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
enum State {
    Remap,
    Filter,
    //Refresh,
    Exit,
}

fn ask_for_field_name() -> String {
    println_cli_info("Please enter the name of the field you wish to filter by:");
    println!();
    println!("{}", "Examples: ".to_string().bright_cyan());
    println!(
        "{} track_metadata.additional_info.duration_ms",
        " - Filter on duration:".to_string().bright_cyan()
    );
    println!(
        "{} track_metadata.mbid_mapping.recording_name",
        " - Filter on mapped recording:".to_string().bright_cyan()
    );
    println!();

    Text::new("").prompt().unwrap()
}

fn ask_for_field_value() -> serde_json::Value {
    loop {
        let unchecked = Text::new(
            "What value does this field have? Text values must be in between \"quotes\": ",
        )
        .prompt()
        .unwrap();

        match serde_json::from_str(&unchecked) {
            Ok(val) => {
                //println!("{val:#?}");
                return val;
            }
            Err(err) => {
                println_cli_err("Couldn't convert the value to a json value. Check for typos? \n - Hint: Text values must be in quotes");

                eprint!("{err}");
                println!();
            }
        }
    }
}

async fn ask_for_mbid() -> RecordingMBID {
    loop {
        let recording_raw = Text::new("What recording should it be mapped to? (MBID / URL)")
            .prompt()
            .unwrap();

        match MBID::from_string(&recording_raw, MusicbrainzEntityKind::Recording) {
            Ok(val) => {
                if !val.is_recording() {
                    println!("The mbid isn't the one of a recording. Please try again");
                    continue;
                }

                let recording_id = val.unwrap_recording();

                return recording_id
                    .get_or_fetch_primary_mbid_alias()
                    .await
                    .unwrap();
            }
            Err(val) => {
                match val {
                    Error::MBIDStringParsingError => {
                        println!("Couldn't parse the string for any MBID. Please make sure there is one.");
                        continue;
                    }
                    _ => Err(val).unwrap(),
                }
            }
        }
    }
}

#[derive(strum_macros::Display, VariantArray, Debug, PartialEq, Eq, Clone)]
enum ListenActions {
    #[strum(to_string = "📝 - Remap")]
    Remap,

    #[strum(to_string = "⏭️ - Skip")]
    Skip,

    #[strum(to_string = "↩️ - Change the filter")]
    ChangeFilter,

    #[strum(to_string = "❌ - Exit")]
    Exit,
}

#[derive(strum_macros::Display, VariantArray, Debug, PartialEq, Eq, Clone)]
enum FilterActions {
    #[strum(to_string = "➕ - Add a rule")]
    Add,
    #[strum(to_string = "🔄️ - Reset the filter")]
    Reset,
    #[strum(to_string = "➡️ - Remap listens")]
    Remap,
    #[strum(to_string = "❌ - Exit")]
    Exit,
}
