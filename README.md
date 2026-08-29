# sniplib-data

Snippet files for the [`sniplib`](https://github.com/antonsakhanovych/sniplib) CLI. This repo holds the
snippet tree; `sniplib` is a separate tool that reads it.

## Layout

```
sniplib-data/
  snippets/            linked into ~/.sniplib
    rust/dsu.rs
    cpp/segtree.cpp
  README.md            repo-root files stay OUTSIDE snippets/
```

`snippets/` is the only directory that ever gets linked into place. Everything
else (this README, notes, templates, CI config) lives at the repo root as a
sibling of `snippets/`, because `sniplib` treats every subdirectory under its
root as a language.

## Quick setup

```sh
git clone <repo-url> ~/sniplib-data
ln -s ~/sniplib-data/snippets ~/.sniplib
ls ~/.sniplib
```

## Background

`sniplib` resolves its snippet root from `SNIPLIB_HOME` if set, otherwise
`$HOME/.sniplib` (`%USERPROFILE%\.sniplib` on Windows). Filesystem calls follow
symlinks, so that path can be a symlink. Point it at this repo's `snippets/`
subdirectory, not the repo root, so repo-root files are never visible to
`sniplib` and only real language directories appear under `~/.sniplib`.

## Setup

Clone somewhere stable:

```sh
git clone <repo-url> ~/sniplib-data
```

Link the `snippets/` subdirectory specifically:

```sh
ln -s ~/sniplib-data/snippets ~/.sniplib
```

Verify the link points at `snippets/` and only language directories show up:

```sh
ls -la ~/.sniplib
ls ~/.sniplib
```

The first command should show `~/.sniplib -> ~/sniplib-data/snippets`. The second
should list only language directories (`rust`, `cpp`, ...), never `README.md` or
other repo-root files.

`SNIPLIB_HOME` does not need to be set when linking to the default `~/.sniplib`
path this way.

## Gotcha: link `snippets/`, not the repo root

```sh
ln -s ~/sniplib-data ~/.sniplib   # wrong
```

This exposes every repo-root directory as a language. A future `templates/` or
`notes/` folder would then show up in `sniplib list` and `sniplib add` would
write into it. Always link the `snippets/` subdirectory.

## Gotcha: new languages are created through the link

`sniplib add` runs `create_dir_all`, so adding a snippet in a new language works
right after linking with no extra setup. The new directory is created inside
`snippets/` through the symlink.

```sh
sniplib add zig dsu
git -C ~/sniplib-data status
```

Run `git status` in this repo after adding snippets or languages to confirm they
landed where expected.
