# Command-Line Help for `alistral`

This document contains the help content for the `alistral` command-line program.

**Command Overview:**

* [`alistral`↴](#alistral)
* [`alistral bump`↴](#alistral-bump)
* [`alistral bump-down`↴](#alistral-bump-down)
* [`alistral cache`↴](#alistral-cache)
* [`alistral cache load-dump`↴](#alistral-cache-load-dump)
* [`alistral cache clear`↴](#alistral-cache-clear)
* [`alistral compatibility`↴](#alistral-compatibility)
* [`alistral config`↴](#alistral-config)
* [`alistral config blacklist-mapper-msid`↴](#alistral-config-blacklist-mapper-msid)
* [`alistral config set-token`↴](#alistral-config-set-token)
* [`alistral config timeout`↴](#alistral-config-timeout)
* [`alistral config listens`↴](#alistral-config-listens)
* [`alistral config listens refresh-unmapped-listens`↴](#alistral-config-listens-refresh-unmapped-listens)
* [`alistral config default-user`↴](#alistral-config-default-user)
* [`alistral lookup`↴](#alistral-lookup)
* [`alistral mapping`↴](#alistral-mapping)
* [`alistral mapping list-unmapped`↴](#alistral-mapping-list-unmapped)
* [`alistral mapping mapper`↴](#alistral-mapping-mapper)
* [`alistral radio`↴](#alistral-radio)
* [`alistral radio circles`↴](#alistral-radio-circles)
* [`alistral radio rate`↴](#alistral-radio-rate)
* [`alistral radio overdue`↴](#alistral-radio-overdue)
* [`alistral stats`↴](#alistral-stats)

## `alistral`

A CLI app containing a set of useful tools for Listenbrainz

**Usage:** `alistral [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `bump` — bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target
* `bump-down` — bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target
* `cache` — Commands to deal with the local cache
* `compatibility` — 
* `config` — Commands to deal with the app's configuration
* `lookup` — Get detailled information about an entity
* `mapping` — Commands for interacting with listen mappings
* `radio` — Generate radio playlists for you
* `stats` — Shows top statistics for a specific target

###### **Options:**

* `--generate <GENERATOR>`

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `alistral bump`

bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.

bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`

All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.

**Usage:** `alistral bump [OPTIONS] [RECORDING]`

###### **Arguments:**

* `<RECORDING>` — The recording to bump

###### **Options:**

* `-d`, `--duration <DURATION>` — The duration the bump last for (Default: 3 months)
* `-m`, `--multiplier <MULTIPLIER>` — The multiplier added to the score (Default: 1.1)
* `-u`, `--username <USERNAME>`



## `alistral bump-down`

bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.

bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`

All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.

**Usage:** `alistral bump-down [OPTIONS] [RECORDING]`

###### **Arguments:**

* `<RECORDING>` — The recording to bump

###### **Options:**

* `-d`, `--duration <DURATION>` — The duration the bump last for (Default: 3 months)
* `-m`, `--multiplier <MULTIPLIER>` — The multiplier added to the score (Default: 1.1)
* `-u`, `--username <USERNAME>`



## `alistral cache`

Commands to deal with the local cache

**Usage:** `alistral cache <COMMAND>`

###### **Subcommands:**

* `load-dump` — Load a listen dump from the website
* `clear` — Wipe the cache's data



## `alistral cache load-dump`

Load a listen dump from the website

Allows to load an exported dump of you listens. This is often faster than using the app. This also prevent stumbling into LB-1584

You can get a listen dump [here](https://listenbrainz.org/settings/export/)

**Usage:** `alistral cache load-dump <PATH> [USERNAME]`

###### **Arguments:**

* `<PATH>` — Path to the dump file
* `<USERNAME>` — Name of the user to import those listens for



## `alistral cache clear`

Wipe the cache's data

This is useful if you need disk space, or need to manually rebuild in case of corruption

**Usage:** `alistral cache clear <TARGET>`

###### **Arguments:**

* `<TARGET>`

  Possible values: `all`




## `alistral compatibility`

**Usage:** `alistral compatibility <USER_A> <USER_B>`

###### **Arguments:**

* `<USER_A>` — The name of the first user
* `<USER_B>` — The name of the second user



## `alistral config`

Commands to deal with the app's configuration

**Usage:** `alistral config <COMMAND>`

###### **Subcommands:**

* `blacklist-mapper-msid` — Prevent an MSID to appear in the mbid mapper
* `set-token` — Associate an user token to an username. This makes `--token` arguments optional, and prevent always having to insert it
* `timeout` — Prevent the recording to appear on radios for a while. If you're burn out of a track and need it gone, use this
* `listens` — Configuration targeting listen data
* `default-user` — Set the default username



## `alistral config blacklist-mapper-msid`

Prevent an MSID to appear in the mbid mapper

**Usage:** `alistral config blacklist-mapper-msid [OPTIONS] <MSID>`

###### **Arguments:**

* `<MSID>` — The msid to blacklist

###### **Options:**

* `--remove` — Remove it from the blacklist



## `alistral config set-token`

Associate an user token to an username. This makes `--token` arguments optional, and prevent always having to insert it

**Usage:** `alistral config set-token <USERNAME> <TOKEN>`

###### **Arguments:**

* `<USERNAME>` — Name of the user to add the token
* `<TOKEN>` — User token



## `alistral config timeout`

Prevent the recording to appear on radios for a while. If you're burn out of a track and need it gone, use this

**Usage:** `alistral config timeout <RECORDING> <DURATION>`

###### **Arguments:**

* `<RECORDING>` — A string containing a MBID of a recording
* `<DURATION>` — A duration to timeout for



## `alistral config listens`

Configuration targeting listen data

**Usage:** `alistral config listens <COMMAND>`

###### **Subcommands:**

* `refresh-unmapped-listens` — Toggle / Set whether the unmapped listens should be automatically updated when fetching listens



## `alistral config listens refresh-unmapped-listens`

Toggle / Set whether the unmapped listens should be automatically updated when fetching listens

**Usage:** `alistral config listens refresh-unmapped-listens <STATE>`

###### **Arguments:**

* `<STATE>` — What do you want it set to?

  Possible values: `toggle`, `true`, `false`




## `alistral config default-user`

Set the default username

**Usage:** `alistral config default-user <USERNAME>`

###### **Arguments:**

* `<USERNAME>`



## `alistral lookup`

Get detailled information about an entity

**Usage:** `alistral lookup <ENTITY_TYPE> <ID> [USERNAME]`

###### **Arguments:**

* `<ENTITY_TYPE>` — The type of entity to look for

  Possible values: `recording`

* `<ID>` — The id of the entity (Accept URLs)
* `<USERNAME>` — Name of the user to look up stats from



## `alistral mapping`

Commands for interacting with listen mappings

**Usage:** `alistral mapping <COMMAND>`

###### **Subcommands:**

* `list-unmapped` — List all of your unlinked listens
* `mapper` — Easy and faster mapping of recordings



## `alistral mapping list-unmapped`

List all of your unlinked listens

This command will list all your unmapped listens, grouped by similarity. It also gives a link to quickly look up the listen in listenbrainz, and go link it

```text

(1) Paul's Dream (Dune) - Caster -> <https://listenbrainz.org/user/user/?min_ts=1709228551&max_ts=1709228553>

(7) Raise Your Weapon - KLOUD -> <https://listenbrainz.org/user/user/?min_ts=1709824520&max_ts=1709824522>

Total: 8 unlinked recordings

```

> Note: Listens are grouped by "Messybrainz ID" (MSID). This is the way Listenbrainz recognize similar listens > by attributing them the same MSID. Linking a listen will link the others as long as they have the same MSID.

> This also means that the same recording can be shown twice in the list. > For example: "Panic - Dion Timer" won't have the same MSID as "Panic by Dion Timmer", even if they are the same recording.

**Usage:** `alistral mapping list-unmapped [OPTIONS] [USERNAME]`

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




## `alistral mapping mapper`

Easy and faster mapping of recordings.

It goes through each unmapped recordings, and give a few suggested recordings for the mapping. This is the exact same as mapping recording in the web UI.

**Usage:** `alistral mapping mapper [OPTIONS] [USERNAME]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument
* `-s`, `--sort <SORT>` — Sort the listens by type

  Possible values: `none`, `name`, `oldest-listen`




## `alistral radio`

Generate radio playlists for you

**Usage:** `alistral radio [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `circles` — Randomly adds recordings from artists you already listened to
* `rate` — Generate playlists depending on the listen rate of recordings
* `overdue` — Generate playlists based on recording that the user should have listened to by now

###### **Options:**

* `--min-count <MIN_COUNT>` — The minimum count of tracks the radio should add to the playlist. (Default: 50, gets overidden by `--min-duration`)
* `--min-duration <MIN_DURATION>` — The minimum duration the playlist should last for. This accept natural language (Ex: "1 hour 36 mins")



## `alistral radio circles`

Randomly adds recordings from artists you already listened to

**Usage:** `alistral radio circles [OPTIONS] [USERNAME] [TOKEN]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from
* `<TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument

###### **Options:**

* `--unlistened` — Use this flag to only get unlistened recordings. This is great for exploration playlists



## `alistral radio rate`

Generate playlists depending on the listen rate of recordings

This algorythm bases itself on your listen rate of recording to get more forgotten tracks. It takes the recordings with the lowest listen rates, and put them into a playlist

**Usage:** `alistral radio rate [OPTIONS] [USERNAME]`

###### **Arguments:**

* `<USERNAME>` — Name of the user to fetch listens from

###### **Options:**

* `-t`, `--token <TOKEN>` — Your user token.

   You can find it at <https://listenbrainz.org/settings/>. If it's set in the config file, you can ignore this argument
* `--min <MIN>` — Minimum listen count
* `-c`, `--cooldown <COOLDOWN>` — The amount of hours needed to wait after a recording have been given before it is re-suggested

  Default value: `0`



## `alistral radio overdue`

Generate playlists based on recording that the user should have listened to by now

Similar to listen rates, this algorithm calculate the average time between listens, and estimate when the next listen will happen. It then put together a playlist made out of recordings you should have listened by now.

**Usage:** `alistral radio overdue [OPTIONS] [USERNAME]`

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



## `alistral stats`

Shows top statistics for a specific target

Target is the entity type to group the stats by. Currently, those entities stats are implemented:

- Recordings (`recording`)

- Artists (`artist`)

- Releases (`release`)

- Release Groups (`release_group`)

- Works (`work`)

**Usage:** `alistral stats [OPTIONS] <TARGET> [USERNAME]`

###### **Arguments:**

* `<TARGET>` — The type of entity to sort by

  Possible values: `recording`, `artist`, `release`, `release-group`, `work`

* `<USERNAME>` — Name of the user to fetch stats listen from

###### **Options:**

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