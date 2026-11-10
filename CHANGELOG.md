# Changelog

All notable changes to this project are documented here. It is generated from
[Conventional Commits](https://www.conventionalcommits.org) by
[git-cliff](https://git-cliff.org) — do not edit it by hand.

## [0.4.5](https://github.com/sorinirimies/tui-rating/compare/v0.4.4...v0.4.5) — 2026-11-10

### ♻️ Refactor

- Remove dead_code allows the build doesn't need; gate on unused dependencies ([`c6d82bf`](https://github.com/sorinirimies/tui-rating/commit/c6d82bfc908119af8b895451c64803f6ae5546ad))

### 📚 Documentation

- Drop the per-repo agent rules skill; the shared skills cover it ([`671ab6a`](https://github.com/sorinirimies/tui-rating/commit/671ab6a5073e691aee467b3e0f05e5a91392188b))
- This repo's agent rules as a project skill in .agents/skills ([`eaaee6f`](https://github.com/sorinirimies/tui-rating/commit/eaaee6f879266c64be58b853e80de8d60c1ae595))
- **readme:** Add the Audit and Fuzz badges ([`3cca079`](https://github.com/sorinirimies/tui-rating/commit/3cca07956bbf2a612f01fcf19a96af16c4d500a0))
- Move agent rules from .zed/rules.md to AGENTS.md ([`4b6fe47`](https://github.com/sorinirimies/tui-rating/commit/4b6fe47e8faed8be9ce36dee03dfd3339850531e))

### 📦 Dependencies

- Bump tui-nuances to 0.1.11 ([`98da059`](https://github.com/sorinirimies/tui-rating/commit/98da059b9f6a279f13d5fd1faf375facc0fe642e))
- Nightly cargo update 2026-11-10 ([`17b7d66`](https://github.com/sorinirimies/tui-rating/commit/17b7d66c67c13f2b4ac581de4523ab6a19ae6226))
- Nightly cargo update 2026-11-09 ([`a120554`](https://github.com/sorinirimies/tui-rating/commit/a120554b24d3084c0a886e098bb820d32bbe3368))
- Nightly cargo update 2026-11-07 ([`ac18b09`](https://github.com/sorinirimies/tui-rating/commit/ac18b09f1bc985fc53481f8e37cb2a2569b9e860))
- Nightly cargo update 2026-11-06 ([`6d66b13`](https://github.com/sorinirimies/tui-rating/commit/6d66b13fa0bb02c719f6774311b85f1b05a3051d))
- Nightly cargo update 2026-11-05 ([`5fd7cbf`](https://github.com/sorinirimies/tui-rating/commit/5fd7cbf71d69edec06904b966e92f885a8ae14cb))
- Nightly cargo update 2026-11-04 ([`ec1d5ee`](https://github.com/sorinirimies/tui-rating/commit/ec1d5ee380728ae06b54beac3df42ad073ba9e55))
- Nightly cargo update 2026-11-03 ([`6d00066`](https://github.com/sorinirimies/tui-rating/commit/6d000666b93390b27bc8600c1fb41f7410a4ea8c))
- Update Cargo.lock ([`a47bea7`](https://github.com/sorinirimies/tui-rating/commit/a47bea7ec4d19f7251a07d545e85e129eac2633d))
- Update Cargo.lock ([`0084e7c`](https://github.com/sorinirimies/tui-rating/commit/0084e7cefcc9d2cfe6d62e7824795a679724af7b))

### 📦 Build

- Align tooling with the library template ([`2043e58`](https://github.com/sorinirimies/tui-rating/commit/2043e58ea40aeaa15b7ad783fc1e29a41c719152))

### 🔄 CI and release

- **gitea:** Check the MSRV on Gitea too, with the same command as GitHub ([`fc9295d`](https://github.com/sorinirimies/tui-rating/commit/fc9295d2d798861e532af25f55b41858d1b46393))
- Download archives to a file so curl retries a dropped transfer, instead of piping into tar ([`eed2d08`](https://github.com/sorinirimies/tui-rating/commit/eed2d08eef68ad0d5d780655648972ae4cd98691))
- Bump softprops/action-gh-release from 2 to 3 (#5) ([`e15d525`](https://github.com/sorinirimies/tui-rating/commit/e15d52511024afa18916dfa698faba73071ee879))
- The Dependabot sweep asks for a rebase on conflicts and keeps going after a failed merge ([`48ddd02`](https://github.com/sorinirimies/tui-rating/commit/48ddd02ad7fe21ecf69d292c46460d76047043d4))
- Merge green Dependabot PRs, majors included, with a nightly sweep; nightly commits skip ignored lockfiles ([`fb70fad`](https://github.com/sorinirimies/tui-rating/commit/fb70fad285c3da2266a524a8a5b2ce99c3a5c1c9))
- Scan the git history for secrets with gitleaks ([`d097bb8`](https://github.com/sorinirimies/tui-rating/commit/d097bb8269e630c211663c89e79c31be1a72ce9e))

### 🔧 Chores

- Remove AGENTS.md ([`35b9b24`](https://github.com/sorinirimies/tui-rating/commit/35b9b242b4f1f5299cff9eefc695e586d03397ef))

## [0.4.4](https://github.com/sorinirimies/tui-rating/compare/v0.4.2...v0.4.4) — 2026-10-28

### 🐛 Bug fixes

- Resolve Cargo.toml merge markers ([`d88fdba`](https://github.com/sorinirimies/tui-rating/commit/d88fdba79ac4910be3cbcb48f9398eebcf254297))

### 📦 Dependencies

- Update Cargo.lock ([`ab4281d`](https://github.com/sorinirimies/tui-rating/commit/ab4281d0cdf25d6c4e9afdf5f8e5510a6730ae63))
- Update Cargo.lock ([`fc7ed15`](https://github.com/sorinirimies/tui-rating/commit/fc7ed151810b97e2621aba8e018a8d3dd05960fe))
- Bump tui-nuances to 0.1.10 ([`335e4fe`](https://github.com/sorinirimies/tui-rating/commit/335e4fe153c57d34d0d7e99271917c46b1e96471))

## [0.4.2](https://github.com/sorinirimies/tui-rating/compare/v0.4.1...v0.4.2) — 2026-10-26

### ✨ Features

- Enable the tui-nuances theme feature by default ([`edc49d6`](https://github.com/sorinirimies/tui-rating/commit/edc49d6daa4a5a85551b26b26f2e7feb78e1f4ee))

### 📦 Dependencies

- Update Cargo.lock ([`7e700d1`](https://github.com/sorinirimies/tui-rating/commit/7e700d1415dae2676e6a5b0518a00b85d6f03841))

### 🔄 CI and release

- Bump actions/checkout from 4 to 7 (#1) ([`2567035`](https://github.com/sorinirimies/tui-rating/commit/2567035639bac26c061ba8506fc5b51111674794))
- Bump clechasseur/rs-clippy-check from 4 to 5 (#2) ([`5403ac9`](https://github.com/sorinirimies/tui-rating/commit/5403ac94ecd8030e7aab8ba3337b60dabaec140e))
- Bump softprops/action-gh-release from 2 to 3 (#3) ([`34ec2b8`](https://github.com/sorinirimies/tui-rating/commit/34ec2b8b1b0f926f9036dd682df8cc0791897c84))
- **gitea:** Nightly workflow-action bumps ([`ea71882`](https://github.com/sorinirimies/tui-rating/commit/ea71882133f2ef4289a61d0b456589e67963335c))
- Automerge major dependabot bumps too ([`9d70876`](https://github.com/sorinirimies/tui-rating/commit/9d7087631f3beec6778d9bc3ac04935d6d4daea9))
- Automerge green PRs (workflow_run) + dependabot for actions ([`5e62b49`](https://github.com/sorinirimies/tui-rating/commit/5e62b4921fbc5871c82cebaf98d114742a01c5f8))

### 🔧 Chores

- Bump tui-nuances to 0.1.7 ([`b766efc`](https://github.com/sorinirimies/tui-rating/commit/b766efcd14ea3f2f4b60637ffd186ed76ae878fe))

## [0.4.1](https://github.com/sorinirimies/tui-rating/compare/v0.4.0...v0.4.1) — 2026-10-21

### 📦 Dependencies

- Update Cargo.lock ([`727adf6`](https://github.com/sorinirimies/tui-rating/commit/727adf68d37b3b52a5ecfa4a372eea29d7fad108))

### 🔄 CI and release

- Allow SliderState in template-leftover check ([`a490577`](https://github.com/sorinirimies/tui-rating/commit/a490577490841ad4d50e54935d3238a3f05e7133))

## [0.4.0](https://github.com/sorinirimies/tui-rating/compare/v0.3.0...v0.4.0) — 2026-10-20

### ✨ Features

- Add NPS, thumbs, Likert, faces, range, form, trend, slider, compare widgets ([`c9c6895`](https://github.com/sorinirimies/tui-rating/commit/c9c6895fc6718445bb9376361744783a39186aef))

### 🐛 Bug fixes

- Saturate u16 layout offsets (found by fuzzing); add render_widgets fuzz target ([`c147b53`](https://github.com/sorinirimies/tui-rating/commit/c147b534cd5c4b3f60b56e5b36e1bdae1ba79c31))

### 🔄 CI and release

- Install pinned Nushell from github.com release URL (runner can't resolve api.github.com) ([`90058f1`](https://github.com/sorinirimies/tui-rating/commit/90058f10515bf272880db08169d2cf83f4327f6f))
- Nightly update pushes to main directly like tui-spinner (no manual-merge PR) ([`8593a8b`](https://github.com/sorinirimies/tui-rating/commit/8593a8bd70ee9a3ce8ef83a75ffbda3ab90f7d6d))

## [0.3.0](https://github.com/sorinirimies/tui-rating/compare/v0.2.1...v0.3.0) — 2026-10-19

### ✨ Features

- Smooth fill, emoji, bars, badges, histogram, bipolar, presets, mouse/keys, animation ([`6d9e815`](https://github.com/sorinirimies/tui-rating/commit/6d9e8152cb2164a87d53d59b8451f3b6d089e878))

## [0.2.1](https://github.com/sorinirimies/tui-rating/compare/v0.2.0...v0.2.1) — 2026-10-19

### ✨ Features

- Add gradients, value formats, label position, highlight, reversed fill and 5 symbol presets ([`834c7f3`](https://github.com/sorinirimies/tui-rating/commit/834c7f3e911e757336108e597beba396f2c8e8e6))

### 🔄 CI and release

- Install Rust toolchain via action (rustup missing on runner) ([`29d1c7a`](https://github.com/sorinirimies/tui-rating/commit/29d1c7a1111accb41bbf38ffd7e71db62f4e26cb))

### 🔧 Chores

- **justfile:** Align git/release/vhs recipes with pi-tokenburn ([`0bea07c`](https://github.com/sorinirimies/tui-rating/commit/0bea07c921ce527996b8e969a22980699ea2f729))
- **justfile:** Push/pull current branch instead of hardcoded main ([`1cc306e`](https://github.com/sorinirimies/tui-rating/commit/1cc306eeea3055f970b9acd56c689615dd92b868))
- **justfile:** Skip unconfigured remotes in *-all recipes ([`8a5baa8`](https://github.com/sorinirimies/tui-rating/commit/8a5baa81795550272dc85d4b0976eaa5ee21272e))

## [0.2.0](https://github.com/sorinirimies/tui-rating/compare/v0.1.0...v0.2.0) — 2026-10-14

### ✨ Features

- Make theming an optional "theme" feature ([`48f5af8`](https://github.com/sorinirimies/tui-rating/commit/48f5af80e35dfa9e22a002f9afa1b157db46d11a))
- Shared theming via tui-nuances ([`c459568`](https://github.com/sorinirimies/tui-rating/commit/c4595681e1d4e7bf5cc15f9cf2e1eff29dfaeb03))

### 🐛 Bug fixes

- **ci:** Install nushell binary in all gitea workflows ([`88e6396`](https://github.com/sorinirimies/tui-rating/commit/88e639699532c44f2980251c15b9a12df54dc2dd))
- **ci:** Install nushell binary on gitea runner (Node 18 lacks File) ([`e1c91bc`](https://github.com/sorinirimies/tui-rating/commit/e1c91bcdd74d0aa06403e73b8e1d05f96381b506))

### 📚 Documentation

- Re-record demos showing results only; reference tui-nuances version ([`96f9f9e`](https://github.com/sorinirimies/tui-rating/commit/96f9f9ec37cf4b56a796aa10206309b885b95d72))

### 🔄 CI and release

- Prebuilt Nushell, cargo-deny + cargo-audit security workflow; fuzzing; tui-nuances 0.1.4 ([`13e6705`](https://github.com/sorinirimies/tui-rating/commit/13e6705a12ee6e10b067c07517fa0436874f9037))

### 🔧 Chores

- Depend on tui-nuances 0.1.6 from crates.io ([`0a4e16a`](https://github.com/sorinirimies/tui-rating/commit/0a4e16aed00f133662343f2ec5d3aacf181d7878))
- Depend on tui-nuances 0.1.6 ([`5e76777`](https://github.com/sorinirimies/tui-rating/commit/5e7677778e20dd7ead61308b74bb1f64195ac641))
- Depend on tui-nuances 0.1.5 ([`6c627c0`](https://github.com/sorinirimies/tui-rating/commit/6c627c064b6dfdbac077a0272d88118a069ed4d0))

## [0.1.0](https://github.com/sorinirimies/tui-rating/releases/tag/v0.1.0) — 2026-10-14

### ✨ Features

- **examples:** Add examples and VHS demo tapes ([`c1b47cf`](https://github.com/sorinirimies/tui-rating/commit/c1b47cf81c504dc0a1f8570ae7802797ca7b758b))
- Add Rating widget, RatingState and RatingSymbols with tests ([`766078b`](https://github.com/sorinirimies/tui-rating/commit/766078bdc18468c947974fb592e631814aaa9e1a))

### 📚 Documentation

- Add README, contributing guide and CI workflows ([`94960e7`](https://github.com/sorinirimies/tui-rating/commit/94960e7cf4e19f4b517d67f3eb4be59342435569))

### 🔧 Chores

- Add nushell scripts, justfile and git-cliff config ([`88798ae`](https://github.com/sorinirimies/tui-rating/commit/88798ae217c113663d2e4b1a8da595167f9f3207))


