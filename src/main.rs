use std::io;

// NOTE: This program does not trim UTF-8 BOM.
// Use: `sed '1s/^\xEF\xBB\xBF//' <input>` to remove BOM.

#[derive(Debug, PartialEq)]
struct Entry {
    yomi: String,
    left: String,
}

/// 見出し語が送り仮名ありエントリかどうかを判定する。
/// 末尾がASCII小文字で、その一つ前がASCII小文字でない場合に送り仮名ありとみなす。
fn is_okuri_ari(yomi: &str) -> bool {
    match (yomi.chars().nth_back(0), yomi.chars().nth_back(1)) {
        (Some(a), Some(b)) if a.is_ascii_lowercase() && !b.is_ascii_lowercase() => true,
        _ => false,
    }
}

/// 入力行のイテレータを受け取り、ヘッダ行・送り仮名ありエントリ・送り仮名なしエントリに分類する。
fn parse_lines<I: Iterator<Item = String>>(
    lines: I,
) -> (Vec<String>, Vec<Entry>, Vec<Entry>) {
    let mut header_lines: Vec<String> = vec![];
    let mut okuri_ari_entries: Vec<Entry> = vec![];
    let mut okuri_nasi_entries: Vec<Entry> = vec![];
    let mut found_body = false;

    for line in lines {
        if line.starts_with(';') {
            if found_body {
                continue;
            }
            if line != ";; okuri-ari entries." && line != ";; okuri-nasi entries." {
                header_lines.push(line);
            }
        } else {
            found_body = true;
            let mut parts = line.splitn(2, ' ');
            match (parts.next(), parts.next()) {
                (Some(yomi), Some(left)) => {
                    if is_okuri_ari(yomi) {
                        okuri_ari_entries.push(Entry {
                            yomi: yomi.to_string(),
                            left: left.to_string(),
                        });
                    } else {
                        okuri_nasi_entries.push(Entry {
                            yomi: yomi.to_string(),
                            left: left.to_string(),
                        });
                    }
                }
                _ => {
                    // スペースなし行は無視
                }
            }
        }
    }

    (header_lines, okuri_ari_entries, okuri_nasi_entries)
}

/// 同じ見出し語を持つエントリをまとめる。
/// 入力は既にソート済みであること。後から出てきた候補が末尾に並ぶよう、
/// 後のエントリの候補を前のエントリの末尾に追記する。
fn merge_entries(entries: Vec<Entry>) -> Vec<Entry> {
    let mut merged: Vec<Entry> = vec![];
    for entry in entries {
        match merged.last_mut() {
            Some(last) if last.yomi == entry.yomi => {
                // "/候補1/候補2/" の先頭 '/' を除いて末尾に連結する
                // last.left = "/A/B/", entry.left = "/C/" → "/A/B/C/"
                let tail = entry.left.trim_start_matches('/');
                last.left.push_str(tail);
            }
            _ => {
                merged.push(entry);
            }
        }
    }
    merged
}

/// ソート済みの出力行を生成する。
fn build_output(
    header_lines: &[String],
    mut okuri_ari_entries: Vec<Entry>,
    mut okuri_nasi_entries: Vec<Entry>,
) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for line in header_lines {
        output.push(line.clone());
    }

    output.push(";; okuri-ari entries.".to_string());
    // 送り仮名ありは逆順ソート
    okuri_ari_entries.sort_by(|a, b| b.yomi.cmp(&a.yomi));
    let okuri_ari_entries = merge_entries(okuri_ari_entries);
    for entry in okuri_ari_entries {
        output.push(format!("{} {}", entry.yomi, entry.left));
    }

    output.push(";; okuri-nasi entries.".to_string());
    // 送り仮名なしは昇順ソート
    okuri_nasi_entries.sort_by(|a, b| a.yomi.cmp(&b.yomi));
    let okuri_nasi_entries = merge_entries(okuri_nasi_entries);
    for entry in okuri_nasi_entries {
        output.push(format!("{} {}", entry.yomi, entry.left));
    }

    output
}

fn main() {
    let lines = io::stdin().lines().map(|r| r.unwrap());
    let (header_lines, okuri_ari_entries, okuri_nasi_entries) = parse_lines(lines);
    let output = build_output(&header_lines, okuri_ari_entries, okuri_nasi_entries);
    for line in output {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- is_okuri_ari ---

    #[test]
    fn test_is_okuri_ari_true() {
        // 末尾がASCII小文字、その前が非ASCII小文字（ひらがな）
        assert!(is_okuri_ari("あいs"));
        assert!(is_okuri_ari("かk"));
    }

    #[test]
    fn test_is_okuri_ari_false_no_suffix() {
        // 末尾がASCII小文字でない
        assert!(!is_okuri_ari("あいう"));
        assert!(!is_okuri_ari("かんじ"));
    }

    #[test]
    fn test_is_okuri_ari_false_two_ascii() {
        // 末尾2文字がどちらもASCII小文字 → 送り仮名なし扱い
        assert!(!is_okuri_ari("abc"));
        assert!(!is_okuri_ari("ab"));
    }

    #[test]
    fn test_is_okuri_ari_false_single_char() {
        // 1文字だけ（nth_back(1) が None）
        assert!(!is_okuri_ari("a"));
        assert!(!is_okuri_ari("あ"));
    }

    #[test]
    fn test_is_okuri_ari_false_empty() {
        assert!(!is_okuri_ari(""));
    }

    // --- parse_lines ---

    #[test]
    fn test_parse_lines_header_preserved() {
        let input = vec![
            ";; -*- coding: utf-8 -*-".to_string(),
            ";; okuri-ari entries.".to_string(),
            ";; okuri-nasi entries.".to_string(),
        ];
        let (headers, ari, nasi) = parse_lines(input.into_iter());
        assert_eq!(headers, vec![";; -*- coding: utf-8 -*-"]);
        assert!(ari.is_empty());
        assert!(nasi.is_empty());
    }

    #[test]
    fn test_parse_lines_okuri_ari_entry() {
        let input = vec![
            ";; okuri-ari entries.".to_string(),
            "あいs /愛す/".to_string(),
        ];
        let (_, ari, nasi) = parse_lines(input.into_iter());
        assert_eq!(ari.len(), 1);
        assert_eq!(ari[0].yomi, "あいs");
        assert_eq!(ari[0].left, "/愛す/");
        assert!(nasi.is_empty());
    }

    #[test]
    fn test_parse_lines_okuri_nasi_entry() {
        let input = vec![
            ";; okuri-nasi entries.".to_string(),
            "あい /愛/相/".to_string(),
        ];
        let (_, ari, nasi) = parse_lines(input.into_iter());
        assert!(ari.is_empty());
        assert_eq!(nasi.len(), 1);
        assert_eq!(nasi[0].yomi, "あい");
        assert_eq!(nasi[0].left, "/愛/相/");
    }

    #[test]
    fn test_parse_lines_comment_after_body_ignored() {
        // 本文開始後のコメント行は無視される
        let input = vec![
            "あい /愛/".to_string(),
            ";; this comment should be ignored".to_string(),
            "うえ /上/".to_string(),
        ];
        let (headers, _, nasi) = parse_lines(input.into_iter());
        assert!(headers.is_empty());
        assert_eq!(nasi.len(), 2);
    }

    #[test]
    fn test_parse_lines_no_space_line_ignored() {
        let input = vec!["スペースなし行".to_string()];
        let (headers, ari, nasi) = parse_lines(input.into_iter());
        assert!(headers.is_empty());
        assert!(ari.is_empty());
        assert!(nasi.is_empty());
    }

    #[test]
    fn test_parse_lines_candidate_with_spaces() {
        // 候補側にスペースが含まれていても splitn(2) で正しく分割される
        let input = vec!["てすと /テスト/test value/".to_string()];
        let (_, _, nasi) = parse_lines(input.into_iter());
        assert_eq!(nasi[0].left, "/テスト/test value/");
    }

    // --- build_output ---

    #[test]
    fn test_build_output_okuri_ari_sorted_reverse() {
        let ari = vec![
            Entry { yomi: "あs".to_string(), left: "/亜/".to_string() },
            Entry { yomi: "うs".to_string(), left: "/有/".to_string() },
            Entry { yomi: "いs".to_string(), left: "/意/".to_string() },
        ];
        let output = build_output(&[], ari, vec![]);
        // ";; okuri-ari entries." の次から逆順
        assert_eq!(output[0], ";; okuri-ari entries.");
        assert_eq!(output[1], "うs /有/");
        assert_eq!(output[2], "いs /意/");
        assert_eq!(output[3], "あs /亜/");
    }

    #[test]
    fn test_build_output_okuri_nasi_sorted_asc() {
        let nasi = vec![
            Entry { yomi: "うえ".to_string(), left: "/上/".to_string() },
            Entry { yomi: "あい".to_string(), left: "/愛/".to_string() },
            Entry { yomi: "いぬ".to_string(), left: "/犬/".to_string() },
        ];
        let output = build_output(&[], vec![], nasi);
        let nasi_start = output.iter().position(|l| l == ";; okuri-nasi entries.").unwrap();
        assert_eq!(output[nasi_start + 1], "あい /愛/");
        assert_eq!(output[nasi_start + 2], "いぬ /犬/");
        assert_eq!(output[nasi_start + 3], "うえ /上/");
    }

    #[test]
    fn test_build_output_header_comes_first() {
        let headers = vec![
            ";; -*- coding: utf-8 -*-".to_string(),
            ";; some comment".to_string(),
        ];
        let output = build_output(&headers, vec![], vec![]);
        assert_eq!(output[0], ";; -*- coding: utf-8 -*-");
        assert_eq!(output[1], ";; some comment");
        assert_eq!(output[2], ";; okuri-ari entries.");
    }

    #[test]
    fn test_build_output_structure() {
        // 出力に必ずセクションヘッダが含まれる
        let output = build_output(&[], vec![], vec![]);
        assert!(output.contains(&";; okuri-ari entries.".to_string()));
        assert!(output.contains(&";; okuri-nasi entries.".to_string()));
        let ari_pos = output.iter().position(|l| l == ";; okuri-ari entries.").unwrap();
        let nasi_pos = output.iter().position(|l| l == ";; okuri-nasi entries.").unwrap();
        assert!(ari_pos < nasi_pos);
    }

    // --- merge_entries ---

    #[test]
    fn test_merge_entries_no_duplicates() {
        let entries = vec![
            Entry { yomi: "あい".to_string(), left: "/相/愛/".to_string() },
            Entry { yomi: "うえ".to_string(), left: "/上/".to_string() },
        ];
        let merged = merge_entries(entries);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].left, "/相/愛/");
        assert_eq!(merged[1].left, "/上/");
    }

    #[test]
    fn test_merge_entries_duplicates_merged() {
        // "あい /相/愛/" と "あい /藍/" → "あい /相/愛/藍/"
        let entries = vec![
            Entry { yomi: "あい".to_string(), left: "/相/愛/".to_string() },
            Entry { yomi: "あい".to_string(), left: "/藍/".to_string() },
        ];
        let merged = merge_entries(entries);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].yomi, "あい");
        assert_eq!(merged[0].left, "/相/愛/藍/");
    }

    #[test]
    fn test_merge_entries_three_duplicates() {
        let entries = vec![
            Entry { yomi: "あい".to_string(), left: "/相/".to_string() },
            Entry { yomi: "あい".to_string(), left: "/愛/".to_string() },
            Entry { yomi: "あい".to_string(), left: "/藍/".to_string() },
        ];
        let merged = merge_entries(entries);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].left, "/相/愛/藍/");
    }

    #[test]
    fn test_build_output_merges_duplicates() {
        // build_output 経由でもマージされることを確認
        let nasi = vec![
            Entry { yomi: "あい".to_string(), left: "/相/愛/".to_string() },
            Entry { yomi: "あい".to_string(), left: "/藍/".to_string() },
            Entry { yomi: "うえ".to_string(), left: "/上/".to_string() },
        ];
        let output = build_output(&[], vec![], nasi);
        let nasi_start = output.iter().position(|l| l == ";; okuri-nasi entries.").unwrap();
        assert_eq!(output[nasi_start + 1], "あい /相/愛/藍/");
        assert_eq!(output[nasi_start + 2], "うえ /上/");
    }
}
