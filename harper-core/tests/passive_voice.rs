//! Exercise rule registration, user configuration, markup, and character spans.
use harper_core::linting::{LintGroup, LintKind, Linter};
use harper_core::spell::FstDictionary;
use harper_core::{Dialect, Document};

#[test]
fn passive_voice_is_registered_and_configurable() {
    let mut group = LintGroup::new_curated(FstDictionary::curated(), Dialect::American);
    assert!(group.config.is_rule_enabled("PassiveVoice"));
    assert!(group.all_descriptions().contains_key("PassiveVoice"));
    group.config.clear();
    group.config.set_rule_enabled("PassiveVoice", true);
    let doc = Document::new_plain_english_curated("The report was written by Alice.");
    let lints = group.lint(&doc);
    assert_eq!(lints.len(), 1);
    assert_eq!(lints[0].lint_kind, LintKind::Style);
    assert!(lints[0].suggestions.is_empty());
    group.config.set_rule_enabled("PassiveVoice", false);
    assert!(group.lint(&doc).is_empty());
}

#[test]
fn passive_voice_respects_markdown_code_and_character_offsets() {
    let text = "😊 Café: the report was reviewed and approved.\n\n`The file was deleted.`\n\n```text\nThe file was deleted.\n```";
    let doc = Document::new_markdown_default_curated(text);
    let mut group = LintGroup::new_curated(FstDictionary::curated(), Dialect::American);
    group.config.clear();
    group.config.set_rule_enabled("PassiveVoice", true);
    let lints = group.lint(&doc);
    assert_eq!(lints.len(), 1);
    assert_eq!(
        lints[0].span.get_content_string(doc.get_source()),
        "was reviewed and approved"
    );
}

#[test]
fn passive_voice_does_not_cross_inline_code() {
    let doc = Document::new_markdown_default_curated("The file was `already` deleted.");
    let mut group = LintGroup::new_curated(FstDictionary::curated(), Dialect::American);
    group.config.clear();
    group.config.set_rule_enabled("PassiveVoice", true);
    assert!(group.lint(&doc).is_empty());
}
