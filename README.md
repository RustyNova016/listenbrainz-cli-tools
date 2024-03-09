# Listenbrainz CLI Tools

A collection of CLI based tools for Listenbrainz.

# Installing
## Builds
The latest build can be found in the [release](https://github.com/RustyNova016/listenbrainz-cli-tools/releases) tab

## Manual build
```shell
git clone https://github.com/RustyNova016/listenbrainz-cli-tools.git
cd ./listenbrainz-cli-tools
cargo build --target
```

# Tools
## Unlinked listens
To search for your unlinked listens, use:
```shell
listenbrainz-cli-tools unlinked -u <username>
```

This will list all your unlinked listens, grouped by similarity. 
It also gives a link to quickly look up the listen in listenbrainz, and go link it

```
(1) Paul's Dream (Dune) - Caster
    -> https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553
(7) Raise Your Weapon - KLOUD
    -> https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522
```

> Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens 
> by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.
> 
> This also means that the same recording can be shown twice in the list. 
> For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.