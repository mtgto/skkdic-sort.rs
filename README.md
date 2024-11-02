skkdic-sort.rs
====
[skkdictools](https://github.com/skk-dev/skktools)のskkdic-sort.cをRustで書き直したものです。

- UTF-8のみ対応
- 先頭の方にあるコメントは残す (マジックコメントや著作者情報などが入っていることが多いので)

## 使い方

標準入力で受け取り、標準出力にソートした結果を出力します。

```console
skkdic-sort < <infile> > <outfile>
```

## ライセンス

MIT
