//! zlibx_parsers — Rust HTML parsers bridged to Dart via flutter_rust_bridge.
//!
//! Each parser is a synchronous pure function:
//! `parse_xxx(html: String, base_url: String) -> Result<XxxPage, ParseError>`,
//! invoked from Dart and executed on the Rust thread pool by flutter_rust_bridge,
//! replacing the existing `Isolate.run` calls.

pub mod models;
pub mod fulltext;

use flutter_rust_bridge::frb;

/// Phase 1 smoke-test function: a trivial pure function to verify the
/// Rust → Dart bridge is wired end-to-end. Not a real parser.
#[frb]
pub fn parse_hello(name: String) -> String {
    format!("hello, {name} (from zlibx_parsers Rust crate)")
}

/// Phase 2 pilot: parse one page of full-text search results.
///
/// Mirrors `parseFullTextSearchPage` in `vendor/olib_api_plugin/lib/src/fulltext.dart`.
/// Input: the raw HTML string and a base URL for resolving relative cover
/// URLs (empty string = no base URL). Output: a `FullTextSearchPage` with
/// hits, total, and total page count.
///
/// Note: `base_url` is `String` (not `Option<String>`) because FRB v2
/// wraps `Option<String>` parameters in `RustOpaque` without auto-unwrapping,
/// which breaks compilation. Empty string is treated as `None`.
#[frb]
pub fn parse_fulltext_search_page(
    html: String,
    base_url: String,
) -> models::FullTextSearchPage {
    let base = if base_url.is_empty() {
        None
    } else {
        Some(base_url.as_str())
    };
    fulltext::parse_fulltext_search_page(&html, base)
}

/// Extract the session's full-text token from a search page's HTML.
///
/// Mirrors `extractFulltextToken` in `fulltext.dart`. Returns `None` when
/// the page carries no token input, or the value is not an 8-hex token.
#[frb]
pub fn extract_fulltext_token(html: String) -> Option<String> {
    fulltext::extract_fulltext_token(&html)
}

/// Extract the query a results page's search form echoes back.
///
/// Mirrors `extractFulltextQuery` in `fulltext.dart`. Returns `None` when
/// the page carries no `q` input (or its value is empty).
#[frb]
pub fn extract_fulltext_query(html: String) -> Option<String> {
    fulltext::extract_fulltext_query(&html)
}

// FRB-generated bindings (produced by `tool/frb_codegen.sh`; gitignored).
// The codegen injects this `mod` declaration — keep it after the doc
// comments and imports so the crate-level `//!` docs stay at the top.
mod frb_generated;
