# satysfi-hash-editor

## これは何

[SATySFi](https://github.com/gfngfn/SATySFi) のフォントハッシュファイルを編集するためのCLIソフトウェアです。

## 使い方

インストールは `git clone` してきて `cargo install` で行います。

`satysfi-hash-editor set $KEY $VALUE` とすると標準入力からハッシュを読んでエントリを追加します。`cat foobar satysfi-hash-editor set BIZUDPGothic dist/fonts/BIZUDPGothic-Regular.ttfl | tee foobar` などしてください (シェルの仕様で、直接リダイレクトすると正常にパースできません)。 

`satysfi-hash-editor delete $KEY` とすると同じようにしてエントリを削除します。
