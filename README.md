# git-doorman

Deny unwanted text entry to your git index

## how does it work?

git-doorman configures a git clean filter which turns away files containing
unwanted text so they can't be added to the index. This can be used to
annotate changes with a watch word so they aren't accidentally commited.

## setup

Install git-doorman:

```
cargo install git-doorman
```

The command `git-doorman` can also be invoked as `git doorman`. Add the doorman
filter to the global git config with the `global-setup` command:

```
git doorman global-setup
```

Next configure a git repository to use doorman. `cd` to it, then use the
`install` command to add the doorman filter for files matching a given pattern.
If no `--pattern` is specified then `*` is used, which matches all files:

```
git doorman install --pattern '*'
```

By default, `install` updates `.git/info/attributes` in the repository, which
configures the attributes only in that clone. To use doorman in all clones of a
repository, for instance on a team, write the filter config to the
`.gitattributes` file with the `--attributes-file` option:

```
git doorman --attributes-file .gitattributes
```

## uninstall

Remove doorman from .git/info/attributes:

```
git doorman uninstall
```

Remove doorman from a .gitattributes file with `--attributes-file`:

```
git doorman uninstall --attributes-file .gitattributes
```

Remove doorman filter from global git config with `global-cleanup`:

```
git global-cleanup
```
