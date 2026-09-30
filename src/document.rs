// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use std::ops::Range;

const RELEASES_SECTION: &str =
    "## Releases\n\nSee [releases.md](releases.md) for the release history.";
const RELEASES_START: &str = "<!-- bake-readme:releases:start -->";
const RELEASES_END: &str = "<!-- bake-readme:releases:end -->";
const RECENT_RELEASE_COUNT: usize = 3;

struct Heading<'document> {
    title: &'document str,
    level: usize,
    start: usize,
    body_start: usize,
}

/// Cargo package fields used to add a generated entry to the `readme.md` See Also section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageMetadata {
    /// Cargo package name.
    pub name: String,
    /// Optional Cargo package description.
    pub description: Option<String>,
    /// Optional Cargo repository URL.
    pub repository: Option<String>,
}

/// A release entry parsed from the project's `releases.md` file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    /// Release heading, usually a version such as `v1.2.3`.
    pub name: String,
    /// Markdown content beneath the release heading.
    pub notes: String,
}

fn headings(document: &str) -> Vec<Heading<'_>> {
    let mut headings = Vec::new();
    let mut offset = 0;
    let mut fence: Option<(u8, usize)> = None;

    for line in document.split_inclusive('\n') {
        let text = line.trim_end_matches(['\r', '\n']);
        let trimmed = text.trim_start_matches(' ');
        let indentation = text.len() - trimmed.len();
        let marker = trimmed.as_bytes().first().copied().unwrap_or_default();
        let count = trimmed.bytes().take_while(|byte| *byte == marker).count();

        if let Some((open_marker, open_count)) = fence {
            if indentation <= 3
                && marker == open_marker
                && count >= open_count
                && trimmed[count..].trim().is_empty()
            {
                fence = None;
            }
        } else if indentation <= 3
            && matches!(marker, b'`' | b'~')
            && count >= 3
            && (marker != b'`' || !trimmed[count..].contains('`'))
        {
            fence = Some((marker, count));
        } else if indentation == 0
            && marker == b'#'
            && (1..=6).contains(&count)
            && (text.len() == count || text.as_bytes()[count].is_ascii_whitespace())
        {
            let title = trimmed[count..].trim();
            let title = trim_closing_hashes(title);
            headings.push(Heading {
                title,
                level: count,
                start: offset,
                body_start: offset + line.len(),
            });
        }

        offset += line.len();
    }

    headings
}

fn trim_closing_hashes(title: &str) -> &str {
    let without_hashes = title.trim_end_matches('#');
    if without_hashes.len() < title.len() && without_hashes.ends_with([' ', '\t']) {
        without_hashes.trim_end()
    } else {
        title
    }
}

fn trailing_blank_lines_start(document: &str, position: usize) -> Range<usize> {
    let prefix = &document[..position];
    let mut start = position;
    for line in prefix.split_inclusive('\n').rev() {
        if line.trim().is_empty() {
            start -= line.len();
        } else {
            break;
        }
    }
    start..position
}

fn newline(document: &str) -> &'static str {
    if document.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

fn rendered_package_entry(package: &PackageMetadata) -> String {
    let name = package.name.trim();
    let name_link = match package
        .repository
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        Some(repository) => format!("[{name}]({repository})"),
        None => format!("`{name}`"),
    };
    let description = package
        .description
        .as_deref()
        .map(str::split_whitespace)
        .map(|words| words.collect::<Vec<_>>())
        .filter(|words| !words.is_empty())
        .map(|words| format!(" — {}", words.join(" ")))
        .unwrap_or_default();

    format!("- {name_link}{description} <!-- bake-readme:package -->")
}

fn section_end(document: &str, headings: &[Heading<'_>], index: usize) -> usize {
    let heading = &headings[index];
    headings
        .iter()
        .skip(index + 1)
        .find(|next| next.level <= heading.level)
        .map_or(document.len(), |next| next.start)
}

/// Parse the newest versioned release entries, excluding the Unreleased section.
pub fn recent_releases(document: &str) -> Vec<Release> {
    let headings = headings(document);
    let Some((releases_index, releases_heading)) = headings
        .iter()
        .enumerate()
        .find(|(_, heading)| heading.level == 1 && heading.title == "Releases")
    else {
        return Vec::new();
    };
    let releases_end = section_end(document, &headings, releases_index);

    headings
        .iter()
        .enumerate()
        .filter(|(_, heading)| {
            heading.level == 2
                && heading.start >= releases_heading.body_start
                && heading.start < releases_end
                && !heading.title.eq_ignore_ascii_case("Unreleased")
        })
        .take(RECENT_RELEASE_COUNT)
        .map(|(index, heading)| Release {
            name: heading.title.to_owned(),
            notes: document[heading.body_start..section_end(document, &headings, index)]
                .trim()
                .to_owned(),
        })
        .collect()
}

fn rendered_recent_releases(releases: &[Release], line_ending: &str) -> String {
    let mut content = String::from("See [releases.md](releases.md) for the full release history.");

    for release in releases {
        content.push_str(line_ending);
        content.push_str(line_ending);
        content.push_str("### ");
        content.push_str(&release.name);
        if !release.notes.is_empty() {
            content.push_str(line_ending);
            content.push_str(line_ending);
            content.push_str(
                &release
                    .notes
                    .replace("\r\n", "\n")
                    .replace('\n', line_ending),
            );
        }
    }

    content
}

fn marker_line(document: &str, range: Range<usize>, marker: &str) -> Option<(usize, usize)> {
    let mut offset = range.start;
    for line in document[range.clone()].split_inclusive('\n') {
        let content = line.trim_end_matches(['\r', '\n']).trim();
        if content == marker {
            return Some((offset, offset + line.len()));
        }
        offset += line.len();
    }
    None
}

fn replace_generated_releases(
    document: &str,
    body: Range<usize>,
    releases: &[Release],
) -> Option<String> {
    let start = marker_line(document, body.clone(), RELEASES_START);
    let end = marker_line(document, body.clone(), RELEASES_END);
    let line_ending = newline(document);
    let content = rendered_recent_releases(releases, line_ending);

    if let (Some((_, start_end)), Some((end_start, _))) = (start, end) {
        if start_end <= end_start {
            let mut updated = String::with_capacity(document.len() + content.len());
            updated.push_str(&document[..start_end]);
            updated.push_str(&content);
            updated.push_str(line_ending);
            updated.push_str(&document[end_start..]);
            return Some(updated);
        }
        return None;
    }

    // Upgrade the simple link generated by earlier versions of this task.
    if document[body.clone()].trim() == "See [releases.md](releases.md) for the release history." {
        let replacement = format!(
            "{line_ending}{RELEASES_START}{line_ending}{content}{line_ending}{RELEASES_END}{line_ending}{line_ending}"
        );
        let mut updated = String::with_capacity(document.len() + replacement.len());
        updated.push_str(&document[..body.start]);
        updated.push_str(&replacement);
        updated.push_str(&document[body.end..]);
        return Some(updated);
    }

    None
}

/// Add or refresh a marked summary of the most recent releases.
///
/// Project-authored Releases sections remain untouched. Sections generated by
/// this task are marked so later updates can refresh their release entries.
pub fn update_releases_section(document: &str, releases: &[Release]) -> String {
    let document_headings = headings(document);
    if let Some((index, heading)) = document_headings
        .iter()
        .enumerate()
        .find(|(_, heading)| heading.title == "Releases")
    {
        let body = heading.body_start..section_end(document, &document_headings, index);
        return replace_generated_releases(document, body, releases)
            .unwrap_or_else(|| document.to_owned());
    }

    let document = ensure_releases_section(document);
    let generated_headings = headings(&document);
    let Some((index, heading)) = generated_headings
        .iter()
        .enumerate()
        .find(|(_, heading)| heading.title == "Releases")
    else {
        return document;
    };
    let body = heading.body_start..section_end(&document, &generated_headings, index);
    replace_generated_releases(&document, body, releases).unwrap_or(document)
}

fn update_see_also_section(document: &str, package: Option<&PackageMetadata>) -> String {
    let headings = headings(document);
    let Some(package) = package else {
        return document.to_owned();
    };
    let entry = rendered_package_entry(package);
    let newline = newline(document);

    if let Some((index, heading)) = headings
        .iter()
        .enumerate()
        .find(|(_, heading)| heading.title == "See Also")
    {
        let end = section_end(document, &headings, index);
        let body = &document[heading.body_start..end];
        if let Some((line_start, line_end, line_ending)) = body
            .split_inclusive('\n')
            .scan(heading.body_start, |offset, line| {
                let start = *offset;
                *offset += line.len();
                Some((start, *offset, line))
            })
            .find_map(|(start, end, line)| {
                line.contains("<!-- bake-readme:package -->").then(|| {
                    let line_ending = if line.ends_with("\r\n") {
                        "\r\n"
                    } else if line.ends_with('\n') {
                        "\n"
                    } else {
                        ""
                    };
                    (start, end, line_ending)
                })
            })
        {
            let mut updated = String::with_capacity(document.len() + entry.len());
            updated.push_str(&document[..line_start]);
            updated.push_str(&entry);
            updated.push_str(line_ending);
            updated.push_str(&document[line_end..]);
            return updated;
        }

        if let Some(repository) = package.repository.as_deref()
            && body.contains(repository)
        {
            return document.to_owned();
        }

        let mut insertion = heading.body_start;
        while insertion < end {
            let remaining = &document[insertion..end];
            let Some(line_end) = remaining.find('\n').map(|index| insertion + index + 1) else {
                break;
            };
            if document[insertion..line_end].trim().is_empty() {
                insertion = line_end;
            } else {
                break;
            }
        }
        let prefix = &document[..insertion];
        let leading = if insertion == heading.body_start
            && (!prefix.ends_with('\n') || !body.starts_with('\n'))
        {
            newline
        } else {
            ""
        };
        let mut updated = String::with_capacity(document.len() + entry.len() + 4);
        updated.push_str(prefix);
        updated.push_str(leading);
        updated.push_str(&entry);
        updated.push_str(newline);
        updated.push_str(newline);
        updated.push_str(&document[insertion..]);
        return updated;
    }

    let target = headings
        .iter()
        .find(|heading| heading.title == "Contributing");
    let section = format!("## See Also{newline}{newline}{entry}");

    if let Some(target) = target {
        let insertion = trailing_blank_lines_start(document, target.start).start;
        let prefix = &document[..insertion];
        let preceding_newlines = prefix
            .chars()
            .rev()
            .take_while(|character| *character == '\n')
            .count();
        let leading = if insertion == 0 || preceding_newlines >= 2 {
            ""
        } else if preceding_newlines == 1 {
            newline
        } else if newline == "\r\n" {
            "\r\n\r\n"
        } else {
            "\n\n"
        };
        let mut updated = String::with_capacity(document.len() + section.len() + 8);
        updated.push_str(prefix);
        updated.push_str(leading);
        updated.push_str(&section);
        updated.push_str(newline);
        updated.push_str(newline);
        updated.push_str(&document[target.start..]);
        updated
    } else {
        let separator = if document.is_empty() || document.ends_with("\n\n") {
            ""
        } else if document.ends_with('\n') {
            newline
        } else if newline == "\r\n" {
            "\r\n\r\n"
        } else {
            "\n\n"
        };
        let mut updated = String::with_capacity(document.len() + section.len() + 4);
        updated.push_str(document);
        updated.push_str(separator);
        updated.push_str(&section);
        updated.push_str(newline);
        updated
    }
}

/// Add the default Releases section if the document has no real Releases heading.
///
/// The section is inserted before a `See Also` or `Contributing` heading when
/// present; otherwise it is appended. Headings inside fenced code blocks and
/// block quotes are ignored. If a Releases section already exists, the original
/// document is returned byte for byte.
pub fn ensure_releases_section(document: &str) -> String {
    let headings = headings(document);
    if headings.iter().any(|heading| heading.title == "Releases") {
        return document.to_owned();
    }

    let target = headings
        .iter()
        .find(|heading| matches!(heading.title, "See Also" | "Contributing"));

    let newline = newline(document);
    let section = RELEASES_SECTION.replace('\n', newline);

    if let Some(target) = target {
        let insertion = trailing_blank_lines_start(document, target.start).start;
        let prefix = &document[..insertion];
        let preceding_newlines = prefix
            .chars()
            .rev()
            .take_while(|character| *character == '\n')
            .count();
        let leading = if insertion == 0 || preceding_newlines >= 2 {
            ""
        } else if preceding_newlines == 1 {
            newline
        } else {
            if newline == "\r\n" {
                "\r\n\r\n"
            } else {
                "\n\n"
            }
        };
        let mut updated = String::with_capacity(document.len() + section.len() + 8);
        updated.push_str(prefix);
        updated.push_str(leading);
        updated.push_str(&section);
        updated.push_str(newline);
        updated.push_str(newline);
        updated.push_str(&document[target.start..]);
        updated
    } else {
        let separator = if document.is_empty() || document.ends_with("\n\n") {
            ""
        } else if document.ends_with('\n') {
            newline
        } else if newline == "\r\n" {
            "\r\n\r\n"
        } else {
            "\n\n"
        };
        let mut updated =
            String::with_capacity(document.len() + section.len() + separator.len() + 1);
        updated.push_str(document);
        updated.push_str(separator);
        updated.push_str(&section);
        updated.push_str(newline);
        updated
    }
}

/// Add or refresh metadata-derived See Also content and the standard Releases section.
///
/// The generated package entry is marked with an HTML comment so subsequent
/// updates can refresh it while preserving the rest of a project-owned section.
pub fn update_document(document: &str, package: Option<&PackageMetadata>) -> String {
    let updated = update_see_also_section(document, package);
    ensure_releases_section(&updated)
}

/// Add or refresh metadata-derived See Also content and recent release entries.
pub fn update_document_with_releases(
    document: &str,
    package: Option<&PackageMetadata>,
    releases: &[Release],
) -> String {
    let updated = update_see_also_section(document, package);
    update_releases_section(&updated, releases)
}
