# Harper with passive voice detection

This fork integrates [JaredTweed/PassiveVoiceDetector](https://github.com/JaredTweed/PassiveVoiceDetector) into Harper's shared Rust engine. `PassiveVoice` is an opt-in rule under **Style and Redundancy**. Builds of the language server, WebAssembly engine, and editor integrations share the same rule.

Passive voice is a style choice, not a grammar error. The rule highlights likely passive constructions. If the actor is unstated, its warning asks whether that actor matters; if a `by` phrase names the actor, it asks whether active wording would improve emphasis or clarity. It does not rewrite sentences automatically: preserving tense, pronoun case, emphasis, and meaning requires context.

## Build and use

```sh
git clone --branch feat/passive-voice https://github.com/JaredTweed/harper.git
cd harper
cargo build --release -p harper-cli -p harper-ls
./target/release/harper-cli lint --only PassiveVoice 'The report was written by Alice.'
```

For VS Code, point `harper.path` at your compiled `target/release/harper-ls` and restart the language server. The marketplace extension uses its own Harper binary unless you configure this path. Set `harper.linters.PassiveVoice` to `true` to turn the rule on. In language server clients, set `harper-ls.linters.PassiveVoice` to `true`. In `harper.js`, use a WebAssembly binary built from this fork and set `{ PassiveVoice: true }` through `setLintConfig`.

## Detection behavior

The rule covers be/get/become constructions, modal and perfect chains, intervening adverbs and negation, inverted questions, coordinated participles, and conservative reduced relatives. It highlights the auxiliary chain through the participle; a coordinated list receives one highlight.

| Text | Behavior |
| --- | --- |
| The report was written by Alice. | Highlights `was written` |
| Termination is guaranteed on any input. | Asks whether the unstated actor matters |
| Termination is guaranteed on any input by a finite state-space. | Notes that the actor is named and asks about emphasis |
| 4 mL HCl were added to the solution. | Warns conditionally; an irrelevant actor may be omitted |
| A finite state-space guarantees termination on any input. | No passive warning |
| The file may already have been deleted. | Highlights the full verb phrase |
| Was the report written? | Detects the inverted passive |
| Has the report been reviewed? | Highlights the opening auxiliary through the participle |
| The report was reviewed, approved, and published. | One highlight for the list |
| The contract was signed by Alice and canceled by Bob. | Two highlights, one for each participle |
| The report may or may not be approved. | Highlights the full auxiliary chain |
| The report written by Alice was published. | Two passive constructions |
| She has written the report by hand. | No passive warning |
| He got written permission. | No passive warning |
| I've been tired all week. | No passive warning |
| She was left-handed. | No passive warning |
| The price dropped by 10 percent. | No passive warning |
| Getting started with Harper. | No passive warning |

This is a heuristic style rule, not a dependency parser. Its bounded context scans keep processing local. Ambiguous result states, ambiguous `'s` contractions without an explicit agent, and agentless reduced relatives using regular past-tense forms deliberately favor avoiding false positives. For example, `It's broken` and `the door closed` are left alone. A contextual verb tag or lexical verb metadata allows recovery when the dictionary lacks past-participle annotations, but ordinary suffixes alone are insufficient.

The `passive_voice_quality` integration test contains 145 hand-labeled cases across coordination, auxiliary chains, questions, reduced relatives, result states, active constructions, and clause boundaries. It includes all five sentence examples in [Harper issue #1500](https://github.com/Automattic/harper/issues/1500) and checks the exact text highlighted by each warning. These deliberately difficult examples are regression guards, not a representative estimate of accuracy on all English writing. In ambiguous cases such as `The account was opened by staff but closed the next day`, the rule warns only about the clear first passive.

## Verification

```sh
cargo test -p harper-core passive_voice
cargo test -p harper-core --test passive_voice_quality
just test-rust
just format
```

Validation on Rust 1.97.1 includes the complete `just test-rust` suite (including desktop tests), strict Clippy for the core and comments libraries, a `wasm32-unknown-unknown` build check, and debug CLI checks of all five issue examples with the rule both off and enabled. Focused tests cover configuration, Markdown code exclusion, and character spans.

## Licensing

The detector contribution in this fork is provided under Apache License 2.0. Harper's original `LICENSE`, existing notices, and `harper-core` package license declaration remain unchanged. No GPL license file is added to Harper.

The standalone [PassiveVoiceDetector](https://github.com/JaredTweed/PassiveVoiceDetector) repository retains its GPL license and provides `passive_voice.rs` under an Apache 2.0 alternative for Harper integration.
