# Command-Line Help for `listenbrainz-cli-tools`

This document contains the help content for the `listenbrainz-cli-tools` command-line program.

**Command Overview:**

* [`listenbrainz-cli-tools`↴](#listenbrainz-cli-tools)
* [`listenbrainz-cli-tools cache`↴](#listenbrainz-cli-tools-cache)
* [`listenbrainz-cli-tools cache load-dump`↴](#listenbrainz-cli-tools-cache-load-dump)
* [`listenbrainz-cli-tools cache clear`↴](#listenbrainz-cli-tools-cache-clear)
* [`listenbrainz-cli-tools config`↴](#listenbrainz-cli-tools-config)
* [`listenbrainz-cli-tools config blacklist-mapper-msid`↴](#listenbrainz-cli-tools-config-blacklist-mapper-msid)
* [`listenbrainz-cli-tools config set-token`↴](#listenbrainz-cli-tools-config-set-token)
* [`listenbrainz-cli-tools config timeout`↴](#listenbrainz-cli-tools-config-timeout)
* [`listenbrainz-cli-tools lookup`↴](#listenbrainz-cli-tools-lookup)
* [`listenbrainz-cli-tools mapping`↴](#listenbrainz-cli-tools-mapping)
* [`listenbrainz-cli-tools mapping list-unmapped`↴](#listenbrainz-cli-tools-mapping-list-unmapped)
* [`listenbrainz-cli-tools mapping mapper`↴](#listenbrainz-cli-tools-mapping-mapper)
* [`listenbrainz-cli-tools radio`↴](#listenbrainz-cli-tools-radio)
* [`listenbrainz-cli-tools radio circles`↴](#listenbrainz-cli-tools-radio-circles)
* [`listenbrainz-cli-tools radio underrated`↴](#listenbrainz-cli-tools-radio-underrated)
* [`listenbrainz-cli-tools radio rate`↴](#listenbrainz-cli-tools-radio-rate)
* [`listenbrainz-cli-tools radio overdue`↴](#listenbrainz-cli-tools-radio-overdue)
* [`listenbrainz-cli-tools stats`↴](#listenbrainz-cli-tools-stats)

## `listenbrainz-cli-tools`

A CLI app containing a set of useful tools for Listenbrainz

**Usage:** `listenbrainz-cli-tools <COMMAND>`

###### **Subcommands:**

* `cache` — Commands to deal with the local cache
* `config` — Commands to deal with the app's configuration
* `lookup` — Get detailled information about an entity
* `mapping` — Commands for interacting with listen mappings
* `radio` — Generate radio playlists for you
* `stats` — Shows top statistics for a specific target



## `listenbrainz-cli-tools cache`

Commands to deal with the local cache

**Usage:** `listenbrainz-cli-tools cache <COMMAND>`

###### **Subcommands:**

* `load-dump` — Load a listen dump from the website
* `clear` — Wipe the cache's data



## `listenbrainz-cli-tools cache load-dump`

Load a listen dump from the website

Allows to load an exported dump of you listens. This is often faster than using the app. This also prevent stumbling into LB-1584

You can get a listen dump [here](https://listenbrainz.org/settings/export/)

**Usage:** `listenbrainz-cli-tools cache load-dump <USERNAME> <PATH>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to import those listens for
* `<PATH>` — Path to the dump file



## `listenbrainz-cli-tools cache clear`

Wipe the cache's data

This is useful if you need disk space, or need to manually rebuild in case of corruption

**Usage:** `listenbrainz-cli-tools cache clear <TARGET>`

###### **Arguments:**

* `<TARGET>`

  Possible values: `all`




## `listenbrainz-cli-tools config`

Commands to deal with the app's configuration

**Usage:** `listenbrainz-cli-tools config <COMMAND>`

###### **Subcommands:**

* `blacklist-mapper-msid` — Prevent an MSID to appear in the mbid mapper
* `set-token` — Associate an user token to an username. This makes `--token` arguments optional, and prevent always having to insert it
* `timeout` — Prevent the recording to appear on radios for a while. If you're burn out of a track and need it gone, use this



## `listenbrainz-cli-tools config blacklist-mapper-msid`

Prevent an MSID to appear in the mbid mapper

**Usage:** `listenbrainz-cli-tools config blacklist-mapper-msid [OPTIONS] <MSID>`

###### **Arguments:**

* `<MSID>` — The msid to blacklist

###### **Options:**

* `--remove` — Remove it from the blacklist



## `listenbrainz-cli-tools config set-token`

Associate an user token to an username. This makes `--token` arguments optional, and prevent always having to insert it

**Usage:** `listenbrainz-cli-tools config set-token <USERNAME> <TOKEN>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to add the token
* `<TOKEN>` — User token



## `listenbrainz-cli-tools config timeout`

Prevent the recording to appear on radios for a while. If you're burn out of a track and need it gone, use this

**Usage:** `listenbrainz-cli-tools config timeout <RECORDING> <DURATION>`

###### **Arguments:**

* `<RECORDING>` — A string containing a MBID of a recording
* `<DURATION>` — A duration to timeout for



## `listenbrainz-cli-tools lookup`

Get detailled information about an entity

**Usage:** `listenbrainz-cli-tools lookup <USERNAME> <ENTITY_TYPE> <ID>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to look up stats from
* `<ENTITY_TYPE>` — The type of entity to look for

  Possible values: `recording`

* `<ID>` — The id of the entity (Accept URLs)



## `listenbrainz-cli-tools mapping`

Commands for interacting with listen mappings

**Usage:** `listenbrainz-cli-tools mapping <COMMAND>`

###### **Subcommands:**

* `list-unmapped` — List all of your unlinked listens
* `mapper` — Easy and faster mapping of recordings



## `listenbrainz-cli-tools mapping list-unmapped`

List all of your unlinked listens

This command will list all your unmapped listens, grouped by similarity. It also gives a link to quickly look up the listen in listenbrainz, and go link it

```text

(1) Paul's Dream (Dune) - Caster -> <https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553>

(7) Raise Your Weapon - KLOUD -> <https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522>

Total: 8 unlinked recordings

```

> Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.

> This also means that the same recording can be shown twice in the list. For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.

**Usage:** `listenbrainz-cli-tools mapping list-unmapped [OPTIONS] <USERNAME>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch unlinked listen from

###### **Options:**

* `-s`, `--sort <SORT>` — Sort the listens by type

  Possible values:
  - `count`:
    The count of listens for this element. This is descending by default
  - `name`:
    The name of the associated element
  - `oldest`:
    The oldest element




## `listenbrainz-cli-tools mapping mapper`

Easy and faster mapping of recordings.

It goes through each unmapped recordings, and give a few suggested recordings for the mapping. This is the exact same as mapping recording in the web UI.

**Usage:** `listenbrainz-cli-tools mapping mapper [OPTIONS] <USERNAME>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument
* `-s`, `--sort <SORT>` — Sort the listens by type

  Possible values: `none`, `name`, `oldest-listen`




## `listenbrainz-cli-tools radio`

Generate radio playlists for you

**Usage:** `listenbrainz-cli-tools radio [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `circles` — Randomly adds recordings from artists you already listened to
* `underrated` — Generate a playlist containing your underrated listens
* `rate` — Generate playlists depending on the listen rate of recordings
* `overdue` — Generate playlists based on recording that the user should have listened to by now

###### **Options:**

* `--min-count <MIN_COUNT>` — The minimum count of tracks the radio should add to the playlist. (Default: 50, gets overidden by `--min-duration`)
* `--min-duration <MIN_DURATION>` — The minimum duration the playlist should last for. This accept natural language (Ex: "1 hour 36 mins")



## `listenbrainz-cli-tools radio circles`

Randomly adds recordings from artists you already listened to

**Usage:** `listenbrainz-cli-tools radio circles [OPTIONS] <USERNAME> [TOKEN]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from
* `<TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument

###### **Options:**

* `--unlistened` — Use this flag to only get unlistened recordings. This is great for exploration playlists



## `listenbrainz-cli-tools radio underrated`

Generate a playlist containing your underrated listens

This radio will create a playlist containing all the tracks that you listen to, but seemingly no one else does.

> The mix is made by calculating a score for each listen. This score is composed of two values: > - The rank in the user's top 1000 recording of all time (First place get 100 points, second get 999.9, etc...) > - The percentage of the recording's listens being from the user (Made with this formula: (user listens / worldwide listens) *100)

**Usage:** `listenbrainz-cli-tools radio underrated [OPTIONS] <USERNAME>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument



## `listenbrainz-cli-tools radio rate`

Generate playlists depending on the listen rate of recordings

This algorythm bases itself on your listen rate of recording to get more forgotten tracks. It takes the recordings with the lowest listen rates, and put them into a playlist

**Usage:** `listenbrainz-cli-tools radio rate [OPTIONS] <USERNAME>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument
* `--min-rate <MIN_RATE>` — Minimum listen rate
* `--min-per <MIN_PER>` — Minimum listen rate time range

  Possible values: `year`, `month`

* `--min <MIN>` — Minimum listen count
* `-c`, `--cooldown <COOLDOWN>` — The amount of hours needed to wait after a recording have been given before it is re-suggested

  Default value: `0`



## `listenbrainz-cli-tools radio overdue`

Generate playlists based on recording that the user should have listened to by now

Similar to listen rates, this algorithm calculate the average time between listens, and estimate when the next listen will happen. It then put together a playlist made out of recordings you should have listened by now.

**Usage:** `listenbrainz-cli-tools radio overdue [OPTIONS] <USERNAME>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument
* `--min <MIN>` — Minimum listen count
* `-c`, `--cooldown <COOLDOWN>` — The amount of hours needed to wait after a recording have been given before it is re-suggested

  Default value: `0`
* `-o`, `--overdue-factor` — Sort the recordings by the time overdue / the average time between listens

   Instead of sorting by date, the listens are sorted by how many estimated listens should have happened by now (Time elapsed since last listen / Average time per listens)

  Default value: `false`



## `listenbrainz-cli-tools stats`

Shows top statistics for a specific target

Target is the entity type to group the stats by. Currently, those entities stats are implemented:

- Recordings (`recording`)

- Artists (`artist`)

- Releases (`release`)

- Release Groups (`release_group`)

- Works (`work`)

**Usage:** `listenbrainz-cli-tools stats [OPTIONS] --target <TARGET> <USERNAME>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch stats listen from

###### **Options:**

* `-t`, `--target <TARGET>` — The type of entity to sort by

  Possible values: `recording`, `artist`, `release`, `release-group`, `work`

* `-s`, `--sort <SORT>` — Sort by:

  Default value: `count`

  Possible values:
  - `count`:
    The count of listens for this element. This is descending by default
  - `name`:
    The name of the associated element
  - `oldest`:
    The oldest element




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>