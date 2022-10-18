# Contributing to *linurgy*

**Thank you very much for considering to contribute to this project!**

We welcome any form of contribution:

- New issues (feature requests, bug reports, questions, ideas, ...)
- Pull requests (documentation improvements, code improvements, new features,
  ...)

**Note**: Before you take the time to open a pull request, please open an issue
first.

## Commit messages

Please try to keep your git commit messages in line with [Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0/).

For example:

```gitcommit
feat: non-allocating editor
```

## Add an entry to the changelog

If your contribution changes the behavior of `linurgy` (as opposed to a typo-fix
in the documentation), please update the [`CHANGELOG.md`](CHANGELOG.md) file and
describe your changes. This makes the release process much easier and therefore
helps to get your changes into a new `linurgy` release faster.

The top of the `CHANGELOG` contains an *"unreleased"* section with a few
subsections (Added, Changed, Testing, …). Please add your entry to the subsection
that best describes your change. If a relevant subsection does not yet exist, please
create it.

Entries follow this format:

```md
### Changed

- Short description of what has been changed, see #123 (@user).
- [**BREAKING**] Please prefix any breaking changes.
```

Here, `#123` is the number of the original issue and/or your pull request.
Please replace `@user` by your GitHub username.
