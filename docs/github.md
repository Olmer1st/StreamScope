# Repository-local GitHub identity

Verified during setup: origin is `https://github.com/Olmer1st/StreamScope.git`; Olmer1st is authenticated in gh and has ADMIN permission; adcore-dev has READ permission and remains the globally active account. GitHub user ID is 14262067. This repository's author is Vadim Malkin with `14262067+Olmer1st@users.noreply.github.com`. Authentication identity and commit authorship are separate settings.

The repository-local helper `scripts/git-credential-olmer1st.sh` calls `gh auth token --hostname github.com --user Olmer1st` only when Git requests HTTPS credentials for github.com. It returns the credential over Git's helper protocol, never stores it in this repository, and ignores store/erase requests. A local empty helper entry clears inherited helpers before this helper runs. The configuration lives in .git/config and is not part of a clone.

For a new clone, with Olmer1st already authenticated:

```sh
git config --local user.name 'Vadim Malkin'
git config --local user.email '14262067+Olmer1st@users.noreply.github.com'
git config --local credential.https://github.com.helper ''
git config --local --add credential.https://github.com.helper "!sh $(git rev-parse --show-toplevel)/scripts/git-credential-olmer1st.sh"
git config --local credential.https://github.com.username Olmer1st
git config --local credential.https://github.com.useHttpPath true
git ls-remote origin
```

On this checkout credential selection was compared to the Olmer1st keyring credential without printing either secret, then remote connectivity was verified. A push of real content is not required for this check. Do not use `gh auth switch` for ordinary Git operations here. The repo-local helper is a trusted script: review changes to it before using it.

For GitHub CLI management commands, use `scripts/gh-olmer1st.sh repo view Olmer1st/StreamScope`, or the same wrapper with other gh arguments. This passes the stored account token only in the child environment. Raw gh commands still use the globally active account. No new GitHub login is currently required. If the keyring session expires, run `gh auth login --hostname github.com` for Olmer1st once; never commit credentials. Check pending legal/release gates before a public push.
