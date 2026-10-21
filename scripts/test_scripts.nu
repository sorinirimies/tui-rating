#!/usr/bin/env nu
# Sanity tests for the project's nushell scripts and repo wiring.
# Usage: nu scripts/test_scripts.nu

def check [name: string, ok: bool] {
    if $ok {
        print $"(ansi green)✅ ($name)(ansi reset)"
    } else {
        print $"(ansi red)❌ ($name)(ansi reset)"
    }
    $ok
}

def main [] {
    let root = ($env.FILE_PWD | path dirname)
    cd $root

    mut results: list<bool> = []

    # 1. every script parses
    for f in (glob scripts/*.nu) {
        let name = ($f | path basename)
        let ok = (nu-check $f)
        $results = ($results | append (check $"parses: ($name)" $ok))
    }

    # 2. version.nu matches Cargo.toml
    let cargo_version = (open Cargo.toml | get package.version)
    let script_version = (^nu scripts/version.nu | str trim)
    $results = ($results | append (check "version.nu matches Cargo.toml" ($cargo_version == $script_version)))

    # 3. semver shape
    $results = ($results | append (check "version is semver" ($cargo_version =~ '^[0-9]+\.[0-9]+\.[0-9]+')))

    # 4. every [[example]] has a source file and a tape
    let examples = (open Cargo.toml | get example)
    for ex in $examples {
        $results = ($results | append (check $"example source exists: ($ex.name)" ($ex.path | path exists)))
        $results = ($results | append (check $"example has tape: ($ex.name)" ($"examples/vhs/($ex.name).tape" | path exists)))
    }

    # 5. every tape references an existing example (or the main demo)
    for tape in (glob examples/vhs/*.tape) {
        let name = ($tape | path parse | get stem)
        let ok = ($name == "tui_rating") or ($examples | any {|e| $e.name == $name })
        $results = ($results | append (check $"tape maps to example: ($name)" $ok))
    }

    # 6. justfile only references scripts that exist
    let refs = (open justfile --raw | parse --regex 'nu (?P<p>scripts/[a-z_]+\.nu)' | get p | uniq)
    for r in $refs {
        $results = ($results | append (check $"justfile script exists: ($r)" ($r | path exists)))
    }

    # 7. no leftover template names
    let leftovers = (glob **/*.{rs,nu,md,toml,yml} --exclude [**/target/** **/.git/**] | where {|f| ($f | path basename) != "test_scripts.nu" } | where {|f| (open $f --raw | str contains "tui-slider") })
    $results = ($results | append (check "no leftover slider references" ($leftovers | is-empty)))

    # 8. required project files
    for f in [README.md LICENSE CHANGELOG.md cliff.toml justfile .gitea/workflows/ci.yml .github/workflows/ci.yml] {
        $results = ($results | append (check $"exists: ($f)" ($f | path exists)))
    }

    let failed = ($results | where {|r| not $r } | length)
    print ""
    if $failed > 0 {
        error make { msg: $"($failed) of ($results | length) checks failed" }
    }
    print $"(ansi green)All ($results | length) checks passed(ansi reset)"
}
