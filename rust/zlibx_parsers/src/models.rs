//! Data models for zlibx_parsers.
//!
//! These mirror the Dart models in `vendor/olib_api_plugin/lib/src/book.dart`
//! and `fulltext.dart`. FRB v2 maps each field to the corresponding Dart type:
//! `String` → `String`, `Option<String>` → `String?`, `i64` → `int` (with
//! `--type-64bit-int`), `Vec<T>` → `List<T>`, `bool` → `bool`.

/// One book, mirroring `Book` in `book.dart`.
///
/// Only the fields populated by a given parser are set; the rest remain
/// `None`. `downloadedAt` is `Option<String>` (ISO-8601) rather than a
/// `DateTime` to avoid pulling in `chrono`; fulltext never sets it.
#[derive(Debug, Clone)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub hash: Option<String>,
    pub href: Option<String>,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub year: Option<i64>,
    pub language: Option<String>,
    pub categories: Option<String>,
    pub pages: Option<i64>,
    pub series: Option<String>,
    pub extension: Option<String>,
    pub filesize: Option<i64>,
    pub filesize_string: Option<String>,
    pub interest_score: Option<String>,
    pub read_online_url: Option<String>,
    pub volume: Option<String>,
    pub isbn10: Option<String>,
    pub isbn13: Option<String>,
    pub ipfs_cid: Option<String>,
    pub ipfs_cid_blake2b: Option<String>,
    pub rating_interest: Option<String>,
    pub rating_quality: Option<String>,
    pub readlist_book_id: Option<i64>,
    pub note: Option<String>,
    pub downloaded_at: Option<String>,
}

/// One highlighted (or plain) run inside a [`FullTextHit`] snippet.
///
/// Mirrors `FullTextSnippetPart` in `fulltext.dart`.
#[derive(Debug, Clone)]
pub struct FullTextSnippetPart {
    pub text: String,
    pub highlighted: bool,
}

/// One full-text search hit: the book plus the matching passage.
///
/// Mirrors `FullTextHit` in `fulltext.dart`.
#[derive(Debug, Clone)]
pub struct FullTextHit {
    pub book: Book,
    pub snippet: Vec<FullTextSnippetPart>,
    pub removed: bool,
    pub unavailable_reason: Option<String>,
}

/// One page of full-text search results.
///
/// Mirrors `FullTextSearchPage` in `fulltext.dart`.
#[derive(Debug, Clone)]
pub struct FullTextSearchPage {
    pub hits: Vec<FullTextHit>,
    pub total: i64,
    pub total_pages: i64,
}
