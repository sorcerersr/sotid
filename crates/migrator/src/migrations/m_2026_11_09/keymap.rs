use std::ops::Range;
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::{
    KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN, KEYMAP_ACTION_ARRAY_PATTERN,
    KEYMAP_ACTION_STRING_PATTERN,
};

pub const KEYMAP_PATTERNS: MigrationPatterns = &[
    (KEYMAP_ACTION_STRING_PATTERN, rename_zed_namespace),
    (KEYMAP_ACTION_ARRAY_PATTERN, rename_zed_namespace),
    (KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN, rename_zed_namespace),
];

/// Rewrites `zed::SomeAction` to `sotid::SomeAction` in a keymap action name.
fn rename_zed_namespace(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let node = mat.nodes_for_capture_index(action_name_ix).next()?;
    let range = node.byte_range();
    let action_name = contents.get(range.clone())?;

    let new_name = action_name.strip_prefix("zed::")?;
    Some((range, format!("sotid::{new_name}")))
}
