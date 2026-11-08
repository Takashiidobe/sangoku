#!/usr/bin/env bash
# manage the rust fork as a patch series on an upstream base commit
#   check          verify fork == base + patches/rust/*.patch, library/core untouched
#   core           check, then compile core metadata for targets/lr35902.json
#   export         regenerate patches/rust/ from the fork's net diff against base
#   rebase <ref>   new branch from upstream <ref> (default upstream HEAD) with patches applied
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
fork=$root/third_party/rust
patches=$root/patches/rust
upstream=https://github.com/rust-lang/rust.git
message='Add LR35902 C ABI classification'

git_fork() { git -C "$fork" "$@"; }
base() { tr -d '[:space:]' <"$patches/BASE"; }

patched_tree() {
    local index
    index=$(mktemp -u)
    GIT_INDEX_FILE=$index git -C "$fork" read-tree "$(base)"
    for patch in "$patches"/*.patch; do
        GIT_INDEX_FILE=$index git -C "$fork" apply --cached "$patch"
    done
    GIT_INDEX_FILE=$index git -C "$fork" write-tree
    rm -f "$index"
}

check() {
    local base_sha
    base_sha=$(base)
    git_fork merge-base --is-ancestor "$base_sha" HEAD ||
        { echo "base $base_sha is not an ancestor of the fork HEAD" >&2; return 1; }
    echo "upstream base: $(git_fork log -1 --format='%h %cs %s' "$base_sha")"
    echo "fork commits since base:"
    git_fork log --reverse --format='  %h %s' "$base_sha"..HEAD
    if [ "$(git_fork rev-list --count "$base_sha"..HEAD)" != 1 ]; then
        echo "fork must be exactly one commit on the base; squash with git reset --soft $base_sha" >&2
        return 1
    fi
    echo "fork diff against base:"
    git_fork diff --stat "$base_sha" HEAD | sed 's/^/  /'
    if [ -n "$(git_fork diff --name-only "$base_sha" HEAD -- library)" ]; then
        echo "fork changes library/, which must stay identical to upstream:" >&2
        git_fork diff --name-only "$base_sha" HEAD -- library >&2
        return 1
    fi
    if [ -n "$(git_fork status --porcelain --untracked-files=no)" ]; then
        echo "fork has uncommitted changes" >&2
        return 1
    fi
    if [ "$(patched_tree)" != "$(git_fork rev-parse 'HEAD^{tree}')" ]; then
        echo "fork tree differs from base + patches; run tools/rust_fork.sh export" >&2
        return 1
    fi
    echo "ok: fork is base + $(ls "$patches"/*.patch | wc -l) patch(es)"
}

core() {
    check
    local out
    out=$(mktemp -d)
    rustup run gbc-lr35902 rustc --edition=2024 --crate-type=rlib --crate-name core \
        --target "$root/targets/lr35902.json" -Zunstable-options -Cpanic=immediate-abort \
        --emit=metadata -o "$out/libcore.rmeta" "$fork/library/core/src/lib.rs" \
        -Awarnings
    rm -rf "$out"
    echo "ok: core frontend builds for lr35902"
}

export_patches() {
    local base_sha tree commit
    base_sha=$(base)
    tree=$(git_fork rev-parse 'HEAD^{tree}')
    commit=$(GIT_AUTHOR_NAME=sangoku GIT_AUTHOR_EMAIL=sangoku@localhost \
        GIT_COMMITTER_NAME=sangoku GIT_COMMITTER_EMAIL=sangoku@localhost \
        GIT_AUTHOR_DATE='@0 +0000' GIT_COMMITTER_DATE='@0 +0000' \
        git -C "$fork" commit-tree "$tree" -p "$base_sha" -m "$message")
    rm -f "$patches"/*.patch
    git_fork format-patch -1 --no-signature --zero-commit --stdout "$commit" \
        >"$patches/0001-add-lr35902-c-abi-classification.patch"
    echo "wrote $patches/0001-add-lr35902-c-abi-classification.patch"
}

rebase() {
    local ref=${1:-HEAD} new_base branch
    git_fork fetch --depth=1 "$upstream" "$ref"
    new_base=$(git_fork rev-parse FETCH_HEAD)
    branch=gbc-lr35902-${new_base:0:11}
    git_fork checkout -b "$branch" "$new_base"
    if ! git_fork -c user.name=sangoku -c user.email=sangoku@localhost \
        am --3way "$patches"/*.patch; then
        echo "patch conflict: resolve in third_party/rust, 'git am --continue', then re-run export" >&2
        return 1
    fi
    printf '%s\n' "$new_base" >"$patches/BASE"
    export_patches
    check
    echo "next: tools/rebuild_toolchain.sh, then cargo nextest run --profile extensive --no-fail-fast"
}

case ${1:-check} in
    check) check ;;
    core) core ;;
    export) export_patches ;;
    rebase) rebase "${2:-}" ;;
    *) sed -n 2,6p "$0" >&2; exit 2 ;;
esac
