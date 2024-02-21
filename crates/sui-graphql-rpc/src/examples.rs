// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::anyhow;
use markdown_gen::markdown::{AsMarkdown, Markdown};
use std::io::{BufWriter, Read};
use std::path::PathBuf;

#[derive(Debug)]
pub struct ExampleQuery {
    pub name: String,
    pub contents: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct ExampleQueryGroup {
    pub name: String,
    pub queries: Vec<ExampleQuery>,
    pub _path: PathBuf,
}

const QUERY_EXT: &str = "graphql";

fn regularize_string(s: &str) -> String {
    // Replace underscore with space and make every word first letter uppercase
    s.replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn load_examples() -> anyhow::Result<Vec<ExampleQueryGroup>> {
    let mut buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    buf.push("examples");

    let mut groups = vec![];
    for entry in std::fs::read_dir(buf).map_err(|e| anyhow::anyhow!(e))? {
        let entry = entry.map_err(|e| anyhow::anyhow!(e))?;
        let path = entry.path();
        let group_name = path
            .file_stem()
            .ok_or(anyhow::anyhow!("File stem cannot be read"))?
            .to_str()
            .ok_or(anyhow::anyhow!("File stem cannot be read"))?
            .to_string();

        let mut group = ExampleQueryGroup {
            name: group_name.clone(),
            queries: vec![],
            _path: path.clone(),
        };

        for file in std::fs::read_dir(path).map_err(|e| anyhow::anyhow!(e))? {
            assert!(file.is_ok());
            let file = file.map_err(|e| anyhow::anyhow!(e))?;
            assert!(file.path().extension().is_some());
            let ext = file
                .path()
                .extension()
                .ok_or(anyhow!("File extension cannot be read"))?
                .to_str()
                .ok_or(anyhow!("File extension cannot be read to string"))?
                .to_string();
            assert_eq!(ext, QUERY_EXT, "wrong file extension for example");

            let file_path = file.path();
            let query_name = file_path
                .file_stem()
                .ok_or(anyhow!("File stem cannot be read"))?
                .to_str()
                .ok_or(anyhow!("File extension cannot be read to string"))?
                .to_string();

            let mut contents = String::new();
            let mut fp = std::fs::File::open(file_path.clone()).map_err(|e| anyhow!(e))?;
            fp.read_to_string(&mut contents).map_err(|e| anyhow!(e))?;
            group.queries.push(ExampleQuery {
                name: query_name,
                contents,
                path: file_path,
            });
        }
        group.queries.sort_by(|x, y| x.name.cmp(&y.name));

        groups.push(group);
    }

    groups.sort_by(|x, y| x.name.cmp(&y.name));
    Ok(groups)
}

pub fn generate_markdown() -> anyhow::Result<String> {
    let groups = load_examples()?;

    let mut output = BufWriter::new(Vec::new());
    let mut md = Markdown::new(&mut output);

    md.write("Sui GraphQL Examples".heading(1))
        .map_err(|e| anyhow!(e))?;

    // TODO: reduce multiple loops
    // Generate the table of contents
    for (id, group) in groups.iter().enumerate() {
        let group_name = regularize_string(&group.name);
        let group_name_toc = format!("[{}](#{})", group_name, id);
        md.write(group_name_toc.heading(3))
            .map_err(|e| anyhow!(e))?;

        for (inner, query) in group.queries.iter().enumerate() {
            let inner_id = inner + 0xFFFF * id;
            let inner_name = regularize_string(&query.name);
            let inner_name_toc = format!("&emsp;&emsp;[{}](#{})", inner_name, inner_id);
            md.write(inner_name_toc.heading(4))
                .map_err(|e| anyhow!(e))?;
        }
    }

    for (id, group) in groups.iter().enumerate() {
        let group_name = regularize_string(&group.name);

        let id_tag = format!("<a id={}></a>", id);
        md.write(id_tag.heading(2))
            .map_err(|e| anyhow::anyhow!(e))?;
        md.write(group_name.heading(2))
            .map_err(|e| anyhow::anyhow!(e))?;
        for (inner, query) in group.queries.iter().enumerate() {
            let inner_id = inner + 0xFFFF * id;
            let name = regularize_string(&query.name);

            let id_tag = format!("<a id={}></a>", inner_id);
            md.write(id_tag.heading(3))
                .map_err(|e| anyhow::anyhow!(e))?;
            md.write(name.heading(3)).map_err(|e| anyhow::anyhow!(e))?;

            // Extract all lines that start with `#` and use them as headers
            let mut headers = vec![];
            let mut query_start = 0;
            for (idx, line) in query.contents.lines().enumerate() {
                let line = line.trim();
                if line.starts_with('#') {
                    headers.push(line.trim_start_matches('#'));
                } else if line.starts_with('{') {
                    query_start = idx;
                    break;
                }
            }

            // Remove headers from query
            let query = query
                .contents
                .lines()
                .skip(query_start)
                .collect::<Vec<_>>()
                .join("\n");

            let content = format!("<pre>{}</pre>", query);
            for header in headers {
                md.write(header.heading(4))
                    .map_err(|e| anyhow::anyhow!(e))?;
            }
            md.write(content.quote()).map_err(|e| anyhow::anyhow!(e))?;
        }
    }
    let bytes = output.into_inner().map_err(|e| anyhow::anyhow!(e))?;
    Ok(String::from_utf8(bytes)
        .map_err(|e| anyhow::anyhow!(e))?
        .replace('\\', ""))
}

#[test]
fn test_generate_markdown() {
    use similar::*;
    use std::fs::File;

    let mut buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    buf.push("docs");
    buf.push("examples.md");
    let mut out_file: File = File::open(buf).expect("Could not open examples.md");

    // Read the current content of `out_file`
    let mut current_content = String::new();
    out_file
        .read_to_string(&mut current_content)
        .expect("Could not read examples.md");
    let new_content: String = generate_markdown().expect("Generating examples markdown failed");

    if current_content != new_content {
        let mut res = vec![];
        let diff = TextDiff::from_lines(&current_content, &new_content);
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "---",
                ChangeTag::Insert => "+++",
                ChangeTag::Equal => "   ",
            };
            res.push(format!("{}{}", sign, change));
        }
        panic!("Doc examples have changed. Please run `sui-graphql-rpc generate-examples` to update the docs. Diff: {}", res.join(""));
    }
}
