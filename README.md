# diceroll

TRPG 向けのシンプルなコマンドライン・ダイスローラー兼、数式評価ツールです。ダイス記法を四則演算や比較演算と組み合わせて実行できます。

```console
$ diceroll --expression "2d6 + 3"
2d6 + 3 = 11

$ diceroll --expression "1d20 >= 15" --without-judge
Comparison: 17 >= 15 => success
```

> [!NOTE]
> ダイスの結果はランダムなので、実際の出力値は例と異なります。

## 機能

- `NdM` 形式のダイスロール（例: `3d6`、`1D20`）
- 四則演算: `+`、`-`、`*`、`/`
- 括弧による優先順位の指定
- 比較演算: `==`、`!=`、`<`、`<=`、`>`、`>=`
- 設定可能なクリティカル／ファンブル判定
- 空白を含む式のサポート

## 必要環境

- Rust（Edition 2024 をサポートするバージョン）
- Cargo

[rustup](https://rustup.rs/) を利用すると、Rust と Cargo をまとめてインストールできます。

## インストール

リポジトリを取得してビルドします。

```bash
git clone https://github.com/frodo821/diceroll.git
cd diceroll
cargo install --path .
```

これで Cargo の bin ディレクトリ（通常は `~/.cargo/bin`）に `diceroll` がインストールされます。インストールせずに `cargo run` で実行することもできます。

## 使い方

```text
diceroll [OPTIONS] --expression <EXPRESSION>
```

開発中に Cargo 経由で実行する場合は、`--` の後ろに diceroll のオプションを指定します。

```bash
cargo run -- --expression "3d6 + 2"
```

### オプション

| オプション | 説明 | デフォルト |
| --- | --- | --- |
| `-e`, `--expression <EXPRESSION>` | 評価する式（必須） | — |
| `-c`, `--crit-fumble-thres <N>` | クリティカル／ファンブル判定のしきい値 | `5` |
| `-G`, `--crit-fumble-greater-is-better` | 大きい値を良い結果として判定 | 無効 |
| `-w`, `--without-judge` | クリティカル／ファンブルの表示を省略 | 無効 |
| `-h`, `--help` | ヘルプを表示 | — |
| `-V`, `--version` | バージョンを表示 | — |

`*`、`<`、`>` などがシェルに解釈されないよう、式は引用符で囲むことを推奨します。

## 式の例

### ダイスと四則演算

```bash
diceroll -e "3d6"
diceroll -e "2d8 + 4"
diceroll -e "(1d20 + 5) * 2"
```

乗除算は加減算より先に評価されます。同じ優先順位の演算は左から評価されます。除算は整数除算で、小数部分はゼロ方向へ切り捨てられます。

### 成否判定

比較式を渡すと、両辺を評価して `success` または `failure` を表示します。

```console
$ diceroll -e "1d20 + 3 >= 15" -w
Comparison: 18 >= 15 => success
```

比較演算子は、1 つの式につき 1 個だけ指定できます。

## クリティカル／ファンブル判定

デフォルトでは「小さい値ほど良い」ルールで、しきい値は `5` です。

| 結果 | デフォルト | `--crit-fumble-greater-is-better` 使用時 |
| --- | --- | --- |
| `値 <= しきい値` | Critical | Fumble |
| `値 >= 100 - しきい値` | Fumble | Critical |

たとえば、デフォルト設定では `1`〜`5` が Critical、`95` 以上が Fumble です。大きい値ほど良いシステムでは `-G` を指定して判定を反転します。

```bash
# 大きい値ほど良いルール
diceroll -e "1d100" --crit-fumble-greater-is-better

# しきい値を 10 に変更
diceroll -e "1d100" --crit-fumble-thres 10

# 判定ラベルを表示しない
diceroll -e "1d100" --without-judge
```

通常の数式では評価結果、比較式では比較の左辺に対して判定されます。この判定は常に 1〜100 を基準としているため、d100 以外の式では `--without-judge` が便利です。

## 開発

```bash
# テスト
cargo test

# フォーマット確認
cargo fmt --check

# 静的解析
cargo clippy --all-targets --all-features
```

ソースコードは次のモジュールに分かれています。

- `src/tokenize.rs`: 入力文字列のトークン化
- `src/parser.rs`: 演算子の優先順位を考慮した構文解析
- `src/expression.rs`: 式、比較、ダイスロールの評価
- `src/position.rs`: 入力位置と範囲の管理
- `src/main.rs`: CLI オプション、結果表示、判定処理
