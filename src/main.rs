use std::io;

// NOTE: This program does not trim UTF-8 BOM.
// Use: `sed '1s/^\xEF\xBB\xBF//' <input>` to remove BOM.

#[derive(Debug)]
struct Entry {
    yomi: String,
    left: String,
}

fn main() {
    let mut okuri_ari_entries: Vec<Entry> = vec![];
    let mut okuri_nasi_entries : Vec<Entry> = vec![];
    let mut found_body = false;

    for line_result in io::stdin().lines() {
        let line = line_result.unwrap();
        if line.starts_with(';') {
            if found_body {
                continue;
            }
            if line != ";; okuri-ari entries." && line != ";; okuri-nasi entries." {
                println!("{}", line);
            }
        } else {
            found_body = true;
            let mut parts = line.splitn(2, " ");
            match (parts.next(), parts.next()) {
                (Some(yomi), Some(left)) => {
                    match (yomi.chars().nth_back(0), yomi.chars().nth_back(1)) {
                        (Some(a), Some(b)) if 'a' <= a && a <= 'z' && (b < 'a' || 'z' < b) => {
                            okuri_ari_entries.push(Entry { yomi: yomi.to_string(), left: left.to_string() });
                        }
                        _ => {
                            okuri_nasi_entries.push(Entry { yomi: yomi.to_string(), left: left.to_string() });
                        }
                    }
                }
                _ => {
                    // no space
                }
            }
        }
    }
    println!(";; okuri-ari entries.");
    okuri_ari_entries.sort_by(|a, b| b.yomi.cmp(&a.yomi));
    for entry in okuri_ari_entries {
        println!("{} {}", entry.yomi, entry.left);
    }
    println!(";; okuri-nasi entries.");
    okuri_nasi_entries.sort_by(|a, b| a.yomi.cmp(&b.yomi));
    for entry in okuri_nasi_entries {
        println!("{} {}", entry.yomi, entry.left);
    }
}
