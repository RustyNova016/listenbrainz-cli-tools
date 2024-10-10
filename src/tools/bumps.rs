use std::str::FromStr;

use chrono::{Duration, Utc};
use rust_decimal::Decimal;

use crate::{
    core::entity_traits::{config_file::ConfigFile, mbid::IsMbid},
    models::{
        cli::BumpCLI,
        config::Config,
        data::{
            listenbrainz::user_listens::UserListens,
            musicbrainz::{entity::entity_kind::MusicbrainzEntityKind, mbid::MBID},
        },
    },
    utils::{extensions::chrono_ext::DurationExt, println_cli},
};

pub async fn bump_command(bump: BumpCLI) {
    let username = Config::check_username(&bump.username);

    let recording = match bump.recording {
        Some(val) => MBID::from_string(&val, MusicbrainzEntityKind::Recording)
            .expect("Couldn't parse MBID")
            .unwrap_recording()
            .get_or_fetch_entity()
            .await
            .expect("Couldn't verify MBID"),
        None => UserListens::get_user_with_refresh(&username)
            .await
            .expect("Couldn't fetch the new listens")
            .get_latest_listen()
            .expect("No listens were found")
            .get_recording_data()
            .await
            .unwrap()
            .expect("The latest listen isn't mapped. Canceling"),
    };

    let multiplier = Decimal::from_str(&bump.multiplier.unwrap_or_else(|| "1.1".to_string()))
        .expect("Couldn't parse the multiplier");

    let duration = match bump.duration {
        Some(dur) => Duration::from_human_string(&dur).expect("Couldn't parse the duration."),
        None => Duration::from_human_string("3 months").expect("Couldn't parse the duration."),
    };

    let mut conf = Config::load_or_panic();

    println_cli(format!(
        "Adding bump to {}, giving a {} multiplier for {}",
        recording
            .get_title_with_credits()
            .await
            .expect("Error while getting recording credits"),
        multiplier,
        duration.to_humantime().unwrap()
    ));

    conf.bumps.add_bump(
        recording.id().clone(),
        username,
        multiplier,
        Utc::now() + duration,
    );
    conf.save().unwrap();
}

pub async fn bump_down_command(mut bump: BumpCLI) {
    bump.multiplier = bump.multiplier.or_else(|| Some("0.9".to_string()));
    bump_command(bump).await;
}
