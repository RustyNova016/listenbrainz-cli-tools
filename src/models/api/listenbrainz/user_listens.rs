use clap::builder::Str;

use super::ListenBrainzAPI;

impl ListenBrainzAPI {
    fn fetch_lastest_listens(&self, user: String) {
        // We get the date of the latest listen
        let latest_date = self.listen_cache.get(&user).and_then(|cached| cached.get_latest_cached_listen()).map(|CachedListen| CachedListen.listen_data.listened_at);

        let last_count = 1;
        let last_oldest_ts = chrono::offset::Utc::now();
        while last_count !=0 || latest_date.is_some_and(|date| date < last_oldest_ts) {
            
        }
    }
}