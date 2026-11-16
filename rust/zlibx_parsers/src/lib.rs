//! zlibx_parsers — Rust HTML parsers bridged to Dart via flutter_rust_bridge.
//!
//! Each parser is a synchronous pure function:
//! `parse_xxx(html: String, base_url: String) -> Result<XxxPage, ParseError>`,
//! invoked from Dart and executed on the Rust thread pool by flutter_rust_bridge,
//! replacing the existing `Isolate.run` calls.

pub mod models;
pub mod fulltext;
pub mod article;
pub mod book_detail;
pub mod booklist;
pub mod category;
pub mod download_stats;
pub mod annas_archive;

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

/// Phase 3: parse one page of article search results (the 文章 tab).
///
/// Mirrors `parseArticleSearchPage` in `article_search.dart`.
/// `base_url` uses the empty-string convention (see parse_fulltext_search_page).
#[frb]
pub fn parse_article_search_page(
    html: String,
    base_url: String,
) -> models::ArticleSearchPage {
    let base = if base_url.is_empty() {
        None
    } else {
        Some(base_url.as_str())
    };
    article::parse_article_search_page(&html, base)
}

/// Extract the query an article results page echoes back (wrong-page
/// guard). Mirrors `extractArticleSearchQuery`.
#[frb]
pub fn extract_article_search_query(html: String) -> Option<String> {
    article::extract_article_search_query(&html)
}

/// Phase 3: parse a book detail page's metadata rows.
///
/// Mirrors `parseBookDetailProperties` in `book_detail_html.dart`.
/// Missing rows parse to None; never throws.
#[frb]
pub fn parse_book_detail_properties(html: String) -> models::BookDetailMetadata {
    book_detail::parse_book_detail_properties(&html)
}

/// Phase 3: parse a /booklists (or /booklists/my) list page.
///
/// Mirrors `parseBooklistsPage` (total_pages 0 when absent).
#[frb]
pub fn parse_booklists_page(
    html: String,
    base_url: String,
) -> models::BooklistsPage {
    booklist::parse_booklists_page(&html, &base_url)
}

/// Phase 3: booklists on one user's profile page.
///
/// Mirrors `parseUserProfileBooklists`.
#[frb]
pub fn parse_user_profile_booklists(
    html: String,
    base_url: String,
) -> Vec<models::Booklist> {
    booklist::parse_user_profile_booklists(&html, &base_url)
}

/// Phase 3: books on the first page of a booklist detail page.
///
/// Mirrors `parseBooklistDetailBooks`.
#[frb]
pub fn parse_booklist_detail_books(
    html: String,
    base_url: Option<String>,
) -> Vec<models::Book> {
    let base = base_url.as_deref().filter(|s| !s.is_empty());
    booklist::parse_booklist_detail_books(&html, base)
}

/// Phase 3: parse a category page.
///
/// Mirrors `parseCategoryBooksPage` (total_pages None when absent,
/// search slug percent-decoded).
#[frb]
pub fn parse_category_books_page(
    html: String,
    base_url: Option<String>,
) -> models::CategoryPage {
    let base = base_url.as_deref().filter(|s| !s.is_empty());
    category::parse_category_books_page(&html, base)
}

/// Phase 3: parse the site's download history page.
///
/// Mirrors `parseDownloadStatsPage` (downloaded_at as ISO-8601 string).
#[frb]
pub fn parse_download_stats_page(html: String) -> Vec<models::Book> {
    download_stats::parse_download_stats_page(&html)
}

// ──────────────────────────────────────────────────────────────────
// Phase 4: Anna's Archive (external source)
// ──────────────────────────────────────────────────────────────────

/// Phase 4: parse an Anna's Archive search results page.
/// Mirrors `ArchiveParsing.parseSearchResults`.
#[frb]
pub fn parse_annas_search_results(
    html: String,
    base_url: String,
    file_type: String,
) -> Vec<models::ArchiveBook> {
    annas_archive::parse_search_results(&html, &base_url, &file_type)
}

/// Phase 4: parse one Anna's Archive `/md5/` detail page.
/// Mirrors `ArchiveParsing.parseBookDetails`.
#[frb]
pub fn parse_annas_book_details(
    html: String,
    url: String,
    base_url: String,
) -> Option<models::ArchiveBookDetails> {
    annas_archive::parse_book_details(&html, &url, &base_url)
}

/// Phase 4: extract the direct download link from a slow_download page.
/// Mirrors `ArchiveParsing.extractSlowDownloadLink`.
#[frb]
pub fn extract_annas_slow_download_link(
    html: String,
    page_url: String,
) -> Option<String> {
    annas_archive::extract_slow_download_link(&html, &page_url)
}

/// Phase 4: bot-protection interstitial detection.
/// Mirrors `ArchiveParsing.isChallengeResponse`.
#[frb]
pub fn annas_is_challenge_response(
    status_code: i64,
    body: String,
    cf_mitigated: Option<String>,
) -> bool {
    annas_archive::is_challenge_response(status_code, &body, cf_mitigated.as_deref())
}

/// Phase 4: is this HTML an Anna's Archive page? Mirrors `isArchivePage`.
#[frb]
pub fn annas_is_archive_page(html: String) -> bool {
    annas_archive::is_archive_page(&html)
}

/// Phase 4: emoji/whitespace cleaner. Mirrors `cleanText`.
#[frb]
pub fn annas_clean_text(text: String) -> String {
    annas_archive::clean_text(&text)
}

/// Phase 4: format detection from an info line. Mirrors `formatFromInfo`.
#[frb]
pub fn annas_format_from_info(info: Option<String>) -> String {
    annas_archive::format_from_info(info.as_deref())
}

/// Phase 4: md5 segment from a path or URL. Mirrors `md5FromLink`.
#[frb]
pub fn annas_md5_from_link(url_or_path: String) -> String {
    annas_archive::md5_from_link(&url_or_path)
}

/// Phase 4: RFC-3986 URL resolution against a base. Mirrors `resolveUrl`.
#[frb]
pub fn annas_resolve_url(base_url: String, href: String) -> String {
    annas_archive::resolve_url(&base_url, &href)
}

/// Phase 4: RFC-3986 URL resolution against a full page URL.
/// Mirrors `resolveRelative`.
#[frb]
pub fn annas_resolve_relative(href: String, page_url: String) -> String {
    annas_archive::resolve_relative(&href, &page_url)
}

/// Phase 4: build a /search URL. Mirrors `buildSearchUrl`.
#[frb]
pub fn annas_build_search_url(
    base_url: String,
    query: String,
    content: String,
    sort: String,
    file_type: String,
    language: String,
    year: String,
    year_from: Option<String>,
    year_to: Option<String>,
    author: String,
    publisher: String,
    enable_filters: Option<bool>,
    page: i64,
) -> String {
    annas_archive::build_search_url(
        &base_url,
        &query,
        &content,
        &sort,
        &file_type,
        &language,
        &year,
        year_from.as_deref(),
        year_to.as_deref(),
        &author,
        &publisher,
        enable_filters,
        page,
    )
}

// FRB-generated bindings (produced by `tool/frb_codegen.sh`; gitignored).
// The codegen injects this `mod` declaration — keep it after the doc
// comments and imports so the crate-level `//!` docs stay at the top.
mod frb_generated;
