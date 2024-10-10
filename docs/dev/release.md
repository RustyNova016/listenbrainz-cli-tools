# How to release

Cheatsheet on how to make a release using the current workflow (It's been 3 months and I already forgot)

## Prerequisites

```
cargo install git-cliff
cargo install cargo-bump
```

## On release

- Create a release branch from `develop`

- Switch to branch

- Execute: 
```
git cliff -o CHANGELOG.md
```

- Execute: 
```
cargo bump <version> --git-tag
```

- Merge branch


- Create Release


- Then run manual workflow:
https://github.com/RustyNova016/listenbrainz-cli-tools/actions/workflows/release_manual.yml