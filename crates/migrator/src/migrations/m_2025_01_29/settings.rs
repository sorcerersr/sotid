use collections::HashMap;
use std::{ops::Range, sync::LazyLock};
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::{
    SETTINGS_LANGUAGES_PATTERN, SETTINGS_ROOT_KEY_VALUE_PATTERN,
};

pub const SETTINGS_PATTERNS: MigrationPatterns = &[
    (SETTINGS_ROOT_KEY_VALUE_PATTERN, replace_setting_name),
    (SETTINGS_LANGUAGES_PATTERN, replace_setting_in_languages),
];

fn replace_setting_name(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let setting_capture_ix = query.capture_index_for_name("name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_capture_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_name_range.clone())?;
    let new_setting_name = SETTINGS_STRING_REPLACE.get(&setting_name)?;
    Some((setting_name_range, new_setting_name.to_string()))
}

pub static SETTINGS_STRING_REPLACE: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from_iter([
            (
                "show_inline_completions_in_menu",
                "show_edit_predictions_in_menu",
            ),
            ("show_inline_completions", "show_edit_predictions"),
            (
                "inline_completions_disabled_in",
                "edit_predictions_disabled_in",
            ),
            ("inline_completions", "edit_predictions"),
        ])
    });

fn replace_setting_in_languages(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let setting_capture_ix = query.capture_index_for_name("setting_name")?;
    let setting_name_range = mat
        .nodes_for_capture_index(setting_capture_ix)
        .next()?
        .byte_range();
    let setting_name = contents.get(setting_name_range.clone())?;
    let new_setting_name = LANGUAGE_SETTINGS_REPLACE.get(&setting_name)?;

    Some((setting_name_range, new_setting_name.to_string()))
}

static LANGUAGE_SETTINGS_REPLACE: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from_iter([
            ("show_inline_completions", "show_edit_predictions"),
            (
                "inline_completions_disabled_in",
                "edit_predictions_disabled_in",
            ),
        ])
    });
