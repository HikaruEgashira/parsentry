# Parsentry Call Graph

このドキュメントは、Parsentryのsrc/ディレクトリの関数呼び出しグラフを可視化しています。

## 更新方法

call graphを更新するには、プロジェクトルートで以下のコマンドを実行してください：

```bash
# Call graphを生成（最大深度3レベル）
cargo run -- --root src --call-graph --call-graph-format mermaid --call-graph-output docs/call_graph.md --call-graph-max-depth 3

# より詳細な分析（最大深度5レベル）
cargo run -- --root src --call-graph --call-graph-format mermaid --call-graph-output docs/call_graph_detailed.md --call-graph-max-depth 5

# 特定の関数から開始する場合
cargo run -- --root src --call-graph --call-graph-format mermaid --call-graph-output docs/call_graph.md --call-graph-start-functions "main,analyze_pattern" --call-graph-max-depth 4
```

## その他の出力フォーマット

```bash
# JSON形式で出力
cargo run -- --root src --call-graph --call-graph-format json --call-graph-output docs/call_graph.json

# DOT形式で出力（Graphviz用）
cargo run -- --root src --call-graph --call-graph-format dot --call-graph-output docs/call_graph.dot

# CSV形式で出力
cargo run -- --root src --call-graph --call-graph-format csv --call-graph-output docs/call_graph.csv
```

## Call Graph

以下は、src/ディレクトリの関数呼び出しグラフです（最大深度3レベル）：

```mermaid
graph TD
  N0("escape\nexternal:0")
  N1("trim_start_matches\nexternal:0")
  N2["test_sarif_with_enhanced_properties\nsarif.rs:50"]
  N3{{"apply_env_vars\nconfig.rs:71"}}
  N4("tree_sitter_php\nexternal:0")
  N5["tarjan_scc\ncall_graph.rs:56"]
  N6("cb\nexternal:0")
  N7{{"print_readable\nresponse.rs:98"}}
  N8["get_vuln_specific_info\nanalysis.rs:19"]
  N9("as_bytes\nexternal:0")
  N10("cloned\nexternal:0")
  N11("repeat\nexternal:0")
  N12("analyze_pattern\nanalyzer.rs:167")
  N13("set_head\nexternal:0")
  N14["read_gitignore\nrepo.rs:20"]
  N15["get_response_language_instruction\nmod.rs:3"]
  N16["apply_cli_args\nconfig.rs:96"]
  N17{{"mitre_attack_ids\nresponse.rs:12"}}
  N18("unwrap_or_else\nexternal:0")
  N19("captures\nexternal:0")
  N20{{"guess_mime_type\nsarif.rs:15"}}
  N21("progress_chars\nexternal:0")
  N22["default_call_graph_max_depth\nconfig.rs:3"]
  N23("tree_sitter_cpp\nexternal:0")
  N24["test_sarif_file_export\nsarif.rs:13"]
  N25("capture_names\nexternal:0")
  N26("definition\nexternal:0")
  N27["test_generate_output_filename_safety\nfilename.rs:24"]
  N28("build_full_graph\ncall_graph.rs:21")
  N29("peel_to_commit\nexternal:0")
  N30("remove_dir_all\nexternal:0")
  N31{{"matches\nsecurity_patterns.rs:76"}}
  N32["get_network_related_files\nrepo.rs:20"]
  N33("chars\nexternal:0")
  N34("ok\nexternal:0")
  N35("dedup_by_key\nexternal:0")
  N36["find_and_load_default\nconfig.rs:7"]
  N37("inc\nexternal:0")
  N38["add_file\nparser.rs:11"]
  N39["test_generate_pattern_specific_filename_basic\nfilename.rs:11"]
  N40{{"from_string\nmod.rs:4"}}
  N41("strip_suffix\nexternal:0")
  N42("test_parse_line_number_from_text\nsarif.rs:21")
  N43{{"owasp_categories\nresponse.rs:12"}}
  N44("trim\nexternal:0")
  N45("open\nexternal:0")
  N46(["determine_node_type\ncall_graph.rs:14"])
  N47{{"get_query_content\nparser.rs:57"}}
  N48("var\nexternal:0")
  N49{{"read_to_string\nrepo.rs:11"}}
  N50("skip\nexternal:0")
  N51{{"contains_key\nparser.rs:29"}}
  N52("len\nexternal:0")
  N53["test_serialize_analysis_result\nresponse.rs:31"]
  N54{{"visit_path\nrepo.rs:16"}}
  N55("get\nexternal:0")
  N56("fmt\nexternal:0")
  N57("split\nexternal:0")
  N58("as_str\nexternal:0")
  N59("concat\nexternal:0")
  N60{{"build_with_precedence\nparser.rs:40"}}
  N61("iter\nexternal:0")
  N62{{"join\nmod.rs:21"}}
  N63{{"replace\nfilename.rs:3"}}
  N64("commit\nexternal:0")
  N65("map_or\nexternal:0")
  N66{{"init\nanalyzer.rs:29"}}
  N67{{"to_string\nresponse.rs:5"}}
  N68("rev\nexternal:0")
  N69["test_github_repo_clone_no_auth\nrepo.rs:58"]
  N70("to_str\nexternal:0")
  N71["test_security_patterns_multiple_files\nsecurity_patterns.rs:21"]
  N72{{"visit_files\nrepo.rs:67"}}
  N73("set_file\nexternal:0")
  N74["test_generate_pattern_specific_filename_special_chars\nfilename.rs:13"]
  N75("slice\nexternal:0")
  N76("map\nexternal:0")
  N77("collect\nexternal:0")
  N78{{"print_response\nanalyzer.rs:74"}}
  N79("write\nexternal:0")
  N80("parse\nexternal:0")
  N81{{"default_language\nconfig.rs:3"}}
  N82("thread_sleep\nexternal:0")
  N83("get_mut\nexternal:0")
  N84("to_markdown\nreports.rs:59")
  N85("push\nexternal:0")
  N86("is_file\nexternal:0")
  N87["get_relevant_files\nrepo.rs:15"]
  N88("println\nexternal:0")
  N89("tree_sitter_java\nexternal:0")
  N90["get_analyzer_prompt_template\nmod.rs:19"]
  N91("contains\nexternal:0")
  N92("tree_sitter_rust\nexternal:0")
  N93{{"to_json\nsarif.rs:76"}}
  N94("exists\nexternal:0")
  N95{{"to_args\nconfig.rs:124"}}
  N96("trim_end_matches\nexternal:0")
  N97("strip_prefix\nexternal:0")
  N98("end_byte\nexternal:0")
  N99["test_github_repo_clone_error\nrepo.rs:44"]
  N100("or_insert\nexternal:0")
  N101{{"print_readable\nresponse.rs:98"}}
  N102["default_call_graph_format\nconfig.rs:3"]
  N103("is_dir\nexternal:0")
  N104("filter\nexternal:0")
  N105("name\nexternal:0")
  N106{{"get_patterns\nsecurity_patterns.rs:11"}}
  N107("yellow\nexternal:0")
  N108{{"detect_language\nfile_classifier.rs:11"}}
  N109("ref_heads\nexternal:0")
  N110["test_owasp_analysis_result\nresponse.rs:17"]
  N111{{"parse_ref\nresponse.rs:17"}}
  N112("clone\nexternal:0")
  N113("extension\nexternal:0")
  N114("blue\nexternal:0")
  N115("and_then\nexternal:0")
  N116("first\nexternal:0")
  N117("count\nexternal:0")
  N118("replace\nexternal:0")
  N119("unwrap_or\nexternal:0")
  N120["load_and_parse_patterns\nsecurity_patterns.rs:21"]
  N121("is_some\nexternal:0")
  N122("display\nexternal:0")
  N123["test_generate_output_filename_basic\nfilename.rs:6"]
  N124("create_dir_all\nexternal:0")
  N125("add_issue\nsarif.rs:23")
  N126("find_map\nexternal:0")
  N127("push_str\nexternal:0")
  N128("start_byte\nexternal:0")
  N129{{"generate_json_schema\npattern_generator.rs:119"}}
  N130{{"query\nparser.rs:49"}}
  N131("sort\nexternal:0")
  N132{{"apply_pattern\npattern_generator.rs:76"}}
  N133("file_path\nexternal:0")
  N134("to_owned\nexternal:0")
  N135("tree_sitter_terraform\nexternal:0")
  N136("get_mut\nexternal:0")
  N137("tree_sitter_typescript\nexternal:0")
  N138["test_should_exclude_file\nrepo.rs:32"]
  N139("or\nexternal:0")
  N140("with_capacity\nexternal:0")
  N141("tree_sitter_go\nexternal:0")
  N142("as_ref\nexternal:0")
  N143("push_str\nexternal:0")
  N144("tree_sitter_python\nexternal:0")
  N145{{"to_string\nfilename.rs:3"}}
  N146{{"new\ncall_graph.rs:3"}}
  N147{{"get_pattern_matches\nsecurity_patterns.rs:86"}}
  N148("utf8_error\nexternal:0")
  N149("parse\nexternal:0")
  N150("as_slice\nexternal:0")
  N151("green\nexternal:0")
  N152("to_uppercase\nexternal:0")
  N153{{"insert\nparser.rs:21"}}
  N154("next\nexternal:0")
  N155{{"load_from_file\nconfig.rs:10"}}
  N156("validate\nconfig.rs:85")
  N157("collect\nexternal:0")
  N158("starts_with\nexternal:0")
  N159("capitalize\nexternal:0")
  N160{{"walk_files\nrepo.rs:51"}}
  N161["default_min_confidence\nconfig.rs:3"]
  N162("is_empty\nexternal:0")
  N163("unwrap\nexternal:0")
  N164("retain\nexternal:0")
  N165("min\nexternal:0")
  N166("tree_sitter_c\nexternal:0")
  N167("file_name\nexternal:0")
  N168("take\nexternal:0")
  N169{{"get_iac_prompt_template\nmod.rs:6"}}
  N170("find\nexternal:0")
  N171("file_stem\nexternal:0")
  N172{{"normalize_pattern_desc\nfilename.rs:3"}}
  N173{{"with_node_types\nparser.rs:34"}}
  N174{{"generate_default_config\nconfig.rs:9"}}
  N175("clone\nexternal:0")
  N176("split_once\nexternal:0")
  N177("to_string\nexternal:0")
  N178{{"get_all_definitions_by_name\nparser.rs:68"}}
  N179{{"visit_dirs\nrepo.rs:40"}}
  N180("find\nexternal:0")
  N181("red\nexternal:0")
  N182{{"render\ncall_graph_output.rs:18"}}
  N183{{"load_with_precedence\nconfig.rs:115"}}
  N184("parent\nexternal:0")
  N185("sort_by\nexternal:0")
  N186{{"add_result\nsummary.rs:11"}}
  N187("test_get_relevant_files\nrepo.rs:9")
  N188{{"default\ncall_graph.rs:10"}}
  N189("to_lowercase\nexternal:0")
  N190{{"byte_to_line_number\ncall_graph.rs:6"}}
  N191("node_type\nexternal:0")
  N192{{"filter_by_pattern\ncall_graph_output.rs:56"}}
  N193{{"new\nparser.rs:6"}}
  N194("repo_builder\nexternal:0")
  N195("source\nexternal:0")
  N196("write_fmt\nexternal:0")
  N197("get\nexternal:0")
  N198{{"new\nrepo.rs:3"}}
  N199{{"load_custom_patterns\nsecurity_patterns.rs:87"}}
  N200("keys\nexternal:0")
  N201("id\nexternal:0")
  N202{{"language_name\nfile_classifier.rs:3"}}
  N203("str\nexternal:0")
  N204("split_at\nexternal:0")
  N205{{"test_markdown_summary\nsummary.rs:12"}}
  N206("all\nexternal:0")
  N207("join\nexternal:0")
  N208{{"test_iac_analysis_prompt\nmod.rs:9"}}
  N209("into_iter\nexternal:0")
  N210("set_message\nexternal:0")
  N211("replace_all\nexternal:0")
  N212{{"extract_function_calls\ncall_graph.rs:51"}}
  N213("from_str\nexternal:0")
  N214("is_match\nexternal:0")
  N215("range\nexternal:0")
  N216{{"build\ncall_graph.rs:23"}}
  N217("deserialize\nexternal:0")
  N218("serialize\nexternal:0")
  N219("enumerate\nexternal:0")
  N220{{"query_captures\nparser.rs:51"}}
  N221("format\nexternal:0")
  N222{{"test_analysis_accuracy_prompt\nmod.rs:18"}}
  N223("trim_matches\nexternal:0")
  N224("read_to_string\nexternal:0")
  N225("to_regex\nexternal:0")
  N226{{"regex\nsecurity_patterns.rs:17"}}
  N227("patterns\nexternal:0")
  N228("file_path\nexternal:0")
  N229("insert\nexternal:0")
  N230{{"from_analysis_summary\nsarif.rs:41"}}
  N231("matches\nexternal:0")
  N232("line_number\nexternal:0")
  N233("strip_prefix\nexternal:0")
  N234("set_style\nexternal:0")
  N235("capitalize\nexternal:0")
  N236("as_str\nexternal:0")
  N237["test_default_config\nconfig.rs:14"]
  N238("magenta\nexternal:0")
  N239{{"new\nsummary.rs:3"}}
  N240{{"test_cli_arg_application\nconfig.rs:6"}}
  N241{{"test_prompt_templates\nmod.rs:14"}}
  N242("to_vec\nexternal:0")
  N243{{"get_messages\nmod.rs:7"}}
  N244("to_node\nexternal:0")
  N245{{"empty_graph\ncall_graph.rs:41"}}
  N246{{"test_config_file_loading\nconfig.rs:14"}}
  N247("as_os_str\nexternal:0")
  N248("home_dir\nexternal:0")
  N249("eq_ignore_ascii_case\nexternal:0")
  N250{{"test_config_env_var_application\nconfig.rs:7"}}
  N251("flat_map\nexternal:0")
  N252("save_to_file\nsarif.rs:26")
  N253("try_into\nexternal:0")
  N254("is_error\nexternal:0")
  N255("finish_with_message\nexternal:0")
  N256("regex\nexternal:0")
  N257("parse\nexternal:0")
  N258{{"test_validate_output_directory\nvalidation.rs:9"}}
  N259("tree_sitter_javascript\nexternal:0")
  N260{{"get_syntax_specific_prompt_elements\nmod.rs:41"}}
  N261("strip_prefix\nexternal:0")
  N262("to_str\nexternal:0")
  N263("run\nexternal:0")
  N264("cyan\nexternal:0")
  N265{{"print_with_color\nanalyzer.rs:56"}}
  N266{{"get_all_definitions\nparser.rs:60"}}
  N267("build\nexternal:0")
  N268("append\nexternal:0")
  N269("to_path_buf\nexternal:0")
  N270("finalize\nexternal:0")
  N271("str\nexternal:0")
  N272("new\nexternal:0")
  N273("update_metadata\ncall_graph.rs:46")
  N274["tarjan_visit\ncall_graph.rs:56"]
  N275{{"test_pattern_generation_with_exclude\npattern_generator.rs:21"}}
  N276("head\nexternal:0")
  N277("check\nexternal:0")
  N278("trim\nexternal:0")
  N279("split_whitespace\nexternal:0")
  N280("capture\nexternal:0")
  N281("to_path_buf\nexternal:0")
  N282{{"determine_node_type\ncall_graph.rs:14"}}
  N283("is_alphabetic\nexternal:0")
  N284("map_err\nexternal:0")
  N285["extract_function_calls_regex\ncall_graph.rs:51"]
  N286("error\nexternal:0")
  N287("find_default_config\nconfig.rs:24")
  N288("remote\nexternal:0")
  N289{{"test_security_patterns_basic_types\nsecurity_patterns.rs:21"}}
  N290{{"test_vulnerability_analysis\nresponse.rs:26"}}
  N291("template\nexternal:0")
  N292("end\nexternal:0")
  N293("anyhow\nexternal:0")
  N294{{"test_config_validation\nconfig.rs:13"}}
  N295{{"add_file\nparser.rs:11"}}
  N296("args\nexternal:0")
  N297("env\nexternal:0")
  N298("vars\nexternal:0")
  N299("matches\nexternal:0")
  N300("find\nexternal:0")
  N301("into\nexternal:0")
  N302["test_find_definition_multiple_files\nrepo.rs:15"]
  N303["test_find_definition_single_file\nrepo.rs:25"]
  N304{{"default_model\nconfig.rs:3"}}
  N305("display\nexternal:0")
  N306{{"exclude_pattern\nsecurity_patterns.rs:20"}}
  N307("expect\nexternal:0")
  N308["remove_duplicate_edges\ncall_graph.rs:94"]
  N309("deserialize\nexternal:0")
  N310("test_find_definition_no_match\nrepo.rs:34")
  N311("parse\nexternal:0")
  N312{{"test_config_generation\nconfig.rs:12"}}
  N313("new\nexternal:0")
  N314("to_case\nexternal:0")
  N315("end_line\nexternal:0")
  N316("start_line\nexternal:0")
  N317("get_workdir\nexternal:0")
  N318("to_str\nexternal:0")
  N319{{"build_from_function\ncall_graph.rs:31"}}
  N320("split\nexternal:0")
  N321("repeat\nexternal:0")
  N322("writeln\nexternal:0")
  N323("error\nexternal:0")
  N324("push\nexternal:0")
  N325("build_children\nexternal:0")
  N326("filename\nexternal:0")
  N327("exclude\nexternal:0")
  N328{{"apply_patterns\npattern_generator.rs:60"}}
  N329("finish\nexternal:0")
  N330("ends_with\nexternal:0")
  N331("home_dir\nexternal:0")
  N332("format\nexternal:0")
  N333{{"classify\nfile_classifier.rs:16"}}
  N334("capture\nexternal:0")
  N335("entries\nexternal:0")
  N336("unwrap_or_default\nexternal:0")
  N337{{"test_pattern_compilation\nsecurity_patterns.rs:21"}}
  N338("borrow\nexternal:0")
  N339{{"build_full_graph\ncall_graph.rs:21"}}
  N340("new\nexternal:0")
  N341("last\nexternal:0")
  N342("set_position\nexternal:0")
  N343("insert_str\nexternal:0")
  N344{{"test_pattern_generation_basic\npattern_generator.rs:18"}}
  N345("start\nexternal:0")
  N346("strip_suffix\nexternal:0")
  N347("index\nexternal:0")
  N348("bright_green\nexternal:0")
  N349{{"test_generate_default_config\nconfig.rs:12"}}

  N59 --> N88
  N61 --> N15
  N61 --> N88
  N62 --> N88
  N62 --> N3
  N62 --> N322
  N193 --> N72
  N193 --> N322
  N193 --> N348
  N193 --> N151
  N193 --> N264
  N241 --> N181
  N241 --> N59
  N265 --> N114
  N265 --> N238
  N265 --> N107
  N265 --> N79
  N222 --> N88

%% Metadata:
%% Total nodes: 350
%% Total edges: 1095
%% Languages: rs
```

## 凡例

- 🔵 青いボックス (`[]`): 通常の関数
- 🔸 菱形 (`{{}}`): セキュリティ関連や重要な関数
- 🟡 楕円 (`()`): 外部ライブラリの関数
- 🟢 角丸ボックス (`([])`): 型判定やヘルパー関数

## 統計情報

- **総ノード数**: 350
- **総エッジ数**: 1095
- **言語**: Rust
- **ルート関数数**: 50

---

*最終更新: 2025-06-28*