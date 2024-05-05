# Listenbrainz CLI Tools

A collection of CLI based tools for Listenbrainz.

# Installing
## Builds
The latest build can be found in the [release](https://github.com/RustyNova016/listenbrainz-cli-tools/releases) tab

## Manual build
```shell
git clone https://github.com/RustyNova016/listenbrainz-cli-tools.git
cd ./listenbrainz-cli-tools
cargo build --release
```

# Tools
## Unmapped listens 
To search for your unmapped listens, use:
```shell
listenbrainz-cli-tools unmapped -u <username>
```

This will list all your unmapped listens, grouped by similarity. 
It also gives a link to quickly look up the listen in listenbrainz, and go link it

```
(1) Paul's Dream (Dune) - Caster
    -> https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553
(7) Raise Your Weapon - KLOUD
    -> https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522
Total: 8 unlinked recordings
```

> Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens 
> by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.
> 
> This also means that the same recording can be shown twice in the list. 
> For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.

## True stats

Due to a bug in listenbrainz, album and artist stats aren't counted properly. This tool allow to see the true stats.

## Radio

Currently, only one algorythm is implemented.

### Circles

This algorythm keep your playlist close to the artists you are listening to. The way it generate is as follow:

- Get a random listen
- Get its artist
- Add a random recording made by this artist

There is the option to only get unlistened recordings, making a alternative to ListenBrainz's own discovery playlists.

Usage:
```shell
listenbrainz-cli-tools ratio -u <username> -t <token>
```

Only unlistened:
```shell
listenbrainz-cli-tools ratio -u <username> -t <token> --unlistened
```