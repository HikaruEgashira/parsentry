# PAR Security Analysis Report

![中高信頼度](https://img.shields.io/badge/信頼度-中高-orange) **信頼度スコア: 85**

## 脆弱性タイプ

- `SSRF`

## PAR Policy Analysis

## 詳細解析

本コードはコマンドライン引数から取得したjdbc_urlを検証なしにGlueのJDBC接続（write_dynamic_frame.from_jdbc_conf, spark.read.jdbc）へそのまま渡しており、攻撃者が任意のホスト・ポートへのJDBC接続を引き起こすことでSSRF攻撃を実現できる可能性があります。内部メタデータサービス（169.254.169.254）や社内リソースへのアクセスを誘発されるリスクがあります。

## PoC（概念実証コード）

```text
# 悪意あるjdbc_urlを指定してメタデータサービスへアクセス
python ETL_JOB.py --JOB_NAME test --s3_source_path s3://example-bucket/data.csv --jdbc_url "jdbc:postgresql://169.254.169.254:5432/postgres?user=foo&password=bar"
# 実行によりGlueジョブが169.254.169.254への接続を試み、メタデータ情報を取得
```

## 修復ガイダンス

### jdbc_urlパラメータの検証

- **Required**: jdbc_urlの許可ホストリストを実装し、外部ユーザが制御する値を直接使用しない
- **Guidance**: 予めホワイトリスト化したDBエンドポイントのみを許可し、CLI引数からの接続先指定を廃止するか、正規表現による厳格なバリデーションを行ってください。
- **Priority**: high

## 解析ノート

- getResolvedOptionsでjdbc_urlをそのまま取得
- 検証なしにwrite_dynamic_frame.from_jdbc_conf, spark.read.jdbcで使用
- 攻撃者は任意ホストへJDBC接続を引き起こせる => SSRF
- 推奨: ホワイトリスト, 入力バリデーション

