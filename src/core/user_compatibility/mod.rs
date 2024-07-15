pub mod builder;

use derive_getters::Getters;
use derive_new::new;
use itertools::Itertools;
use rust_decimal::Decimal;
use tokio::sync::OnceCell;

use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listens_with_entity::ListensWithEntity;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::recording::Recording;

use super::display::progress_bar::ProgressBarCli;

#[derive(Getters, new)]
pub struct UserCompatibility {
    user_a: String,
    user_a_listens: ListenCollection,

    user_b: String,
    user_b_listens: ListenCollection,

    shared_recordings: OnceCell<Vec<RecordingMBID>>,
}

pub enum TargetUser {
    UserA,
    UserB,
}

impl UserCompatibility {
    pub async fn get_shared_recordings(&self) -> color_eyre::Result<&Vec<RecordingMBID>> {
        self.shared_recordings
            .get_or_try_init(|| async move {
                let unique_recordings_ids_a = self
                    .get_listened_recordings_mbids(TargetUser::UserA)
                    .await?;
                let unique_recordings_ids_b = self
                    .get_listened_recordings_mbids(TargetUser::UserB)
                    .await?;

                Ok(unique_recordings_ids_a
                    .clone()
                    .into_iter()
                    .filter(|rec| unique_recordings_ids_b.contains(rec))
                    .collect_vec())
            })
            .await
    }

    pub async fn get_listened_recordings_mbids(
        &self,
        target: TargetUser,
    ) -> color_eyre::Result<Vec<RecordingMBID>> {
        match target {
            TargetUser::UserA => self.user_a_listens.get_listened_recordings_mbids().await,
            TargetUser::UserB => self.user_b_listens.get_listened_recordings_mbids().await,
        }
    }

    pub async fn get_user_ratio(
        &self,
        target: TargetUser,
    ) -> color_eyre::Result<Vec<(Decimal, RecordingMBID)>> {
        let shared_recordings = self.get_shared_recordings().await?;
        let user_listens = self
            .get_user_listens(target)
            .clone()
            .try_into_mapped_primary()
            .await?;

        let progress = ProgressBarCli::new(
            shared_recordings.len() as u64,
            Some("Calculating listen ratios"),
        );
        let num_total_listens = Decimal::new(user_listens.len().try_into().unwrap(), 0);
        let mut ratios = Vec::new();

        for shared_rec in shared_recordings {
            let rec_and_listens = ListensWithEntity::<Recording>::from_unfiltered(
                shared_rec
                    .into_stateful()
                    .await?
                    .get_load_or_fetch()
                    .await?,
                &user_listens,
            );

            let ratio = Decimal::new(rec_and_listens.listen_count().try_into().unwrap(), 0)
                / num_total_listens;

            ratios.push((ratio, shared_rec.clone()));
            progress.inc(1);
        }

        Ok(ratios)
    }

    pub fn get_user_listens(&self, target: TargetUser) -> &ListenCollection {
        match target {
            TargetUser::UserA => &self.user_a_listens,
            TargetUser::UserB => &self.user_b_listens,
        }
    }

    pub async fn get_shared_ratio(&self) -> color_eyre::Result<Decimal> {
        let mut total_ratio = Decimal::ZERO;
        let ratios_a = self.get_user_ratio(TargetUser::UserA).await?;
        let ratios_b = self.get_user_ratio(TargetUser::UserB).await?;

        for rec in self.get_shared_recordings().await? {
            let Some(ratio_a) = ratios_a.iter().find(|(_, id)| id == rec) else {
                continue;
            };
            let Some(ratio_b) = ratios_b.iter().find(|(_, id)| id == rec) else {
                continue;
            };

            if ratio_a.0 < ratio_b.0 {
                total_ratio += ratio_a.0;
            } else {
                total_ratio += ratio_b.0;
            }
        }

        Ok(total_ratio)
    }

    pub async fn get_user_shared_percent(&self, target: TargetUser) -> color_eyre::Result<Decimal> {
        Ok(Decimal::new(
            self.get_shared_recordings()
                .await?
                .len()
                .try_into()
                .unwrap(),
            0,
        ) / Decimal::new(
            self.get_listened_recordings_mbids(target)
                .await?
                .len()
                .try_into()
                .unwrap(),
            0,
        ) * Decimal::ONE_HUNDRED)
    }
}
