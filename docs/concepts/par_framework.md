# PAR Framework

PAR（Principal-Action-Resource）は、セキュリティ脆弱性を体系的に分類・検出するための解析モデルです。

## 3つの要素

### Principal（主体）

操作を開始するエンティティ。信頼できないデータの入口点を表します。

- ユーザー入力（フォーム、API パラメータ）
- 外部システムからのデータ
- 環境変数、設定ファイル
- ネットワーク経由のデータ

### Action（操作）

データに対して実行される処理。検証や変換を含みます。

- データ検証・サニタイズ
- 認証・認可チェック
- 暗号化・ハッシュ化
- 型変換・正規化

### Resource（リソース）

操作の対象となるシステムリソース。攻撃の最終目標となる箇所です。

- データベース操作
- ファイルシステム
- コマンド実行
- ネットワーク通信

## 従来手法との違い

従来の静的解析は Source（入力）→ Sink（出力）のデータフロー追跡に焦点を当てます。

PAR はこれを拡張し：

- **Principal** は Source に加え、その信頼レベルを考慮
- **Action** は中間処理の品質（検証の有無・妥当性）を評価
- **Resource** は Sink を一般化し、影響範囲を明確化

これにより、単なるデータフローではなく「誰が」「何を経て」「何に」アクセスするかという文脈で脆弱性を理解できます。

## パイプラインでの活用

Parsentry のパイプライン（model → scan → generate）において、PAR 分類は脅威モデルの生成とプロンプト構築で活用されます：

1. **Principal 特定**: `parsentry model` が生成する脅威モデルプロンプトで、リポジトリのエントリポイント（APIハンドラ、入力フォーム等）を Principal として列挙
2. **Action・Resource の文脈化**: `parsentry scan` が各 AttackSurface のソースコードを読み込み、Principal→Action→Resource の経路を外部エージェントが分析するためのコンテキストを構築
3. **結果の集約**: `parsentry generate` が SARIF v2.1.0 形式で結果を集約し、各脆弱性を PAR 分類に沿って報告

## 例

```javascript
function processUser(req) {
    const id = req.params.id;           // Principal: ユーザー入力
    // Action: 検証なし
    db.query(`SELECT * FROM users WHERE id = ${id}`);  // Resource: DB
}
```

この例では：
- **P**: HTTP リクエストパラメータ（信頼できない）
- **A**: 検証・サニタイズが欠如
- **R**: データベースクエリ（SQL インジェクションのリスク）

PAR の視点により、Action の欠如が脆弱性の原因であることが明確になります。

Parsentry では、このコードが AttackSurface として列挙され、外部エージェントが P→A→R の経路を分析して SARIF 形式で結果を出力します。
