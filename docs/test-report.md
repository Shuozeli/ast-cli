# ast-cli Test Report

Generated: 2026-03-27

Tested against **24 real source files** across 5 languages from local project repos.

### Summary

| Language   | Files | Outline | Read | Skeleton Avg |
|------------|------:|:-------:|:----:|:------------:|
| Rust       |     6 | 6/6     | 5/5  | 64%          |
| C++        |     4 | 4/4     | 2/2  | 33%          |
| TypeScript |     5 | 5/5     | 3/3  | 60%          |
| Python     |     5 | 5/5     | 1/1  | 59%          |
| Protobuf   |     5 | 5/5     | 3/3  | 0% (no bodies)|

**Skeleton reduction** varies by content: function-heavy files get 70-93%, declaration-only
files (C headers, proto, type-only TS) get 0% (correct -- nothing to strip).

### Known Behaviors

- **0% skeleton reduction** on files with no function bodies (C headers, .proto, pure type definitions) -- expected
- **`use <unknown>`** for Rust use declarations -- use paths are tree nodes, not named; cosmetic only
- **Protobuf skeleton 0%** -- proto has no function bodies; skeleton equals original

---

## 1. Outline + Skeleton

### Rust

| File | Lines | Items | Skeleton | Reduction |
|------|------:|------:|---------:|----------:|
| flatbuffers reader.rs                      |   773 |  101 |   128 |  84% |
| flatbuffers schema lib.rs                  |   643 |   64 |   531 |  18% |
| flatbuffers binary_walker.rs               |  1040 |  100 |   180 |  83% |
| flatbuffers tests.rs                       |   435 |   19 |   105 |  76% |
| bevy world/mod.rs                          |  4642 |  405 |  2810 |  40% |
| ast-agent ast_ops.rs                       |  1395 |   91 |   285 |  80% |

### C++

| File | Lines | Items | Skeleton | Reduction |
|------|------:|------:|---------:|----------:|
| sqliteInt.h (C header)                     |  5898 |  714 |  5898 |   0% |
| rapidjson document.h                       |  2575 |  408 |  1977 |  24% |
| grpc_rust_generator.cc                     |   953 |   54 |   174 |  82% |
| grpc_rust_generator.h                      |   102 |   15 |    78 |  24% |

### TypeScript

| File | Lines | Items | Skeleton | Reduction |
|------|------:|------:|---------:|----------:|
| PageAgentCore.ts                           |   641 |   18 |   163 |  75% |
| types.ts (interfaces only)                 |   289 |   14 |   289 |   0% |
| ConfigPanel.tsx                            |   363 |    2 |    29 |  93% |
| text-buffer.ts                             |  4007 |  141 |   877 |  79% |
| hub-ws.ts                                  |   243 |   26 |   110 |  55% |

### Python

| File | Lines | Items | Skeleton | Reduction |
|------|------:|------:|---------:|----------:|
| test_mcp_server_integration.py             |  1082 |   39 |   210 |  81% |
| test_bd_client.py                          |   714 |   40 |   218 |  70% |
| test_tools.py                              |   490 |   24 |   142 |  72% |
| test_mcp_compaction.py                     |   432 |   34 |   124 |  72% |
| annotations.py (no functions)              |  2569 |  177 |  2569 |   0% |

### Protobuf

| File | Lines | Items | Skeleton | Reduction |
|------|------:|------:|---------:|----------:|
| context_forge.proto                        |   303 |   39 |   303 |   0% |
| av_proxy.proto                             |   776 |  104 |   776 |   0% |
| db_control_plane.proto                     |   388 |   57 |   388 |   0% |
| k8s generated.proto                        |  7305 |  240 |  7305 |   0% |
| company_service.proto                      |    80 |   11 |    80 |   0% |

## 2. Read (:: addressing)

### Rust

| Address | Status | First Line |
|---------|--------|------------|
| `World::spawn` | OK | `pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut<'_> {` |
| `World` | OK | `pub struct World {` |
| `BinaryWalker::walk` | OK | `pub fn walk(mut self, root_type_name: &str) -> Result<Vec<AnnotatedReg` |
| `BaseType` | OK | `pub enum BaseType {` |
| `list_items` | OK | `pub fn list_items(source: &str) -> Result<Vec<Item>> {` |

### C++

| Address | Status | First Line |
|---------|--------|------------|
| `GenericMemberIterator` | OK | `class GenericMemberIterator` |
| `100:115` | OK | `  if (path_to_message_module == "self::") {` |

### TypeScript

| Address | Status | First Line |
|---------|--------|------------|
| `PageAgentCore` | OK | `class PageAgentCore extends EventTarget {` |
| `PageAgentCore::stop` | OK | `stop() {` |
| `HubWs` | OK | `class HubWs {` |

### Python

| Address | Status | First Line |
|---------|--------|------------|
| `test_list_issues_tool` | OK | `@pytest.mark.asyncio` |
| `TestBdClient` | N/A | (no classes in file -- all top-level functions) |

### Protobuf

| Address | Status | First Line |
|---------|--------|------------|
| `ContextForgeService` | OK | `service ContextForgeService {` |
| `PullContextRequest` | OK | `message PullContextRequest {` |
| `DatabaseStatus` | OK | `enum DatabaseStatus {` |

## 3. Find (cross-file search)

| Symbol | Kind | Directory | Matches |
|--------|------|-----------|--------:|
| `spawn` | `function` | `src/` | 19 |
| `walk` | `function` | `flatbuffers-rs/` | 1 |
| `PageAgentCore` | `` | `src/` | 1 |
| `test_list_issues_tool` | `function` | `tests/` | 1 |
| `ContextForgeService` | `` | `dragb/` | 1 |

## 4. Query (tree-sitter S-expressions)

| Query | Matches |
|-------|--------:|
| Rust: functions starting with 'spawn' | 12 |
| Python: test_ functions | 36 |
| TypeScript: class methods | 6 |
| Protobuf: RPC methods | 4 |
| C: struct definitions | 134 |

## 5. Project (directory summary)

```
9 files, 1532 lines

  /home/cyuan/projects/ast-cli/src/languages/mod.rs        rust   102 lines    7 fn    1 types   0 tests
  /home/cyuan/projects/ast-cli/src/main.rs                 rust   134 lines    1 fn    3 types   0 tests
  /home/cyuan/projects/ast-cli/src/ops/find.rs             rust   110 lines    4 fn    1 types   0 tests
  /home/cyuan/projects/ast-cli/src/ops/mod.rs              rust     6 lines    0 fn    0 types   0 tests
  /home/cyuan/projects/ast-cli/src/ops/outline.rs          rust   552 lines   27 fn    1 types   0 tests
  /home/cyuan/projects/ast-cli/src/ops/project.rs          rust   152 lines    5 fn    2 types   0 tests
  /home/cyuan/projects/ast-cli/src/ops/query.rs            rust    68 lines    2 fn    1 types   0 tests
  /home/cyuan/projects/ast-cli/src/ops/read.rs             rust   328 lines   13 fn    0 types   0 tests
  /home/cyuan/projects/ast-cli/src/ops/skeleton.rs         rust    80 lines    4 fn    0 types   0 tests
```

## 6. Outline Samples

### Rust (bevy world/mod.rs, first 30 lines)
```
module command_queue [8:8]
module deferred_world [9:9]
module entity_access [10:10]
module entity_fetch [11:11]
module filtered_resource [12:12]
module identifier [13:13]
module spawn_batch [14:14]
module error [16:16]
module reflect [18:18]
module unsafe_world_cell [19:19]
use <unknown> [21:24]
use <unknown> [25:25]
use <unknown> [26:26]
use <unknown> [27:31]
use <unknown> [32:32]
use <unknown> [33:33]
use <unknown> [34:34]
use <unknown> [35:35]
use <unknown> [37:70]
use <unknown> [71:71]
use <unknown> [72:72]
use <unknown> [73:73]
use <unknown> [74:74]
use <unknown> [75:75]
use <unknown> [76:76]
use <unknown> [77:77]
struct World [98:115]
impl Default for World [117:142]
  function default [118:141]  -- fn default () -> Self
impl Drop for World [144:155]
```

### TypeScript (PageAgentCore.ts)
```
export type_alias PageAgentCoreConfig [29:29]
export class PageAgentCore [61:641]
  method constructor [97:146]  -- constructor (config: PageAgentCoreConfig)
  method status [149:151]  -- get status () : AgentStatus
  method #emitStatusChange [154:156]  -- #emitStatusChange () : void
  method #emitHistoryChange [159:161]  -- #emitHistoryChange () : void
  method #emitActivity [167:169]  -- #emitActivity (activity: AgentActivity) : void
  method #setStatus [172:177]  -- #setStatus (status: AgentStatus) : void
  method pushObservation [185:187]  -- pushObservation (content: string) : void
  method stop [190:194]  -- stop ()
  method execute [196:349]  -- async execute (task: string) : Promise<ExecutionResult>
  method #packMacroTool [360:443]  -- #packMacroTool () : Tool<MacroToolInput, MacroToolResult>
  method #getSystemPrompt [448:460]  -- #getSystemPrompt () : string
  method #getInstructions [465:504]  -- async #getInstructions () : Promise<string>
  method #handleObservations [511:550]  -- async #handleObservations (step: number) : Promise<void>
  method #assembleUserPrompt [552:620]  -- async #assembleUserPrompt () : Promise<string>
  method #onDone [622:627]  -- #onDone (success = true)
  method dispose [629:640]  -- dispose ()
```

### Python (test_mcp_server_integration.py, first 20)
```
function bd_executable [13:22]  -- def bd_executable()
function temp_db [25:61]  -- def temp_db(bd_executable)
function mcp_client [64:97]  -- def mcp_client(bd_executable, temp_db, monkeypatch)
function test_quickstart_resource [100:108]  -- def test_quickstart_resource(mcp_client)
function test_create_issue_tool [111:136]  -- def test_create_issue_tool(mcp_client)
function test_show_issue_tool [139:157]  -- def test_show_issue_tool(mcp_client)
function test_list_issues_tool [160:182]  -- def test_list_issues_tool(mcp_client)
function test_update_issue_tool [185:213]  -- def test_update_issue_tool(mcp_client)
function test_close_issue_tool [216:238]  -- def test_close_issue_tool(mcp_client)
function test_reopen_issue_tool [241:267]  -- def test_reopen_issue_tool(mcp_client)
function test_reopen_multiple_issues_tool [270:300]  -- def test_reopen_multiple_issues_tool(mcp_client)
function test_reopen_with_reason_tool [303:328]  -- def test_reopen_with_reason_tool(mcp_client)
function test_ready_work_tool [331:369]  -- def test_ready_work_tool(mcp_client)
function test_add_dependency_tool [372:397]  -- def test_add_dependency_tool(mcp_client)
function test_create_with_all_fields [400:423]  -- def test_create_with_all_fields(mcp_client)
function test_list_with_filters [426:466]  -- def test_list_with_filters(mcp_client)
function test_ready_work_with_priority_filter [469:485]  -- def test_ready_work_with_priority_filter(mcp_client)
function test_update_partial_fields [488:514]  -- def test_update_partial_fields(mcp_client)
function test_dependency_types [517:541]  -- def test_dependency_types(mcp_client)
function test_stats_tool [544:564]  -- def test_stats_tool(mcp_client)
```

### Protobuf (context_forge.proto)
```
service ContextForgeService [4:9]
enum SourceId [13:22]
enum ContextKind [24:41]
message PullContextRequest [45:49]
message PullContextResponse [51:60]
message ContextFragment [64:85]
message Participant [89:92]
message EarningsTranscriptPayload [94:100]
message FinancialReport [102:106]
message FinancialStatementsPayload [108:112]
message PriceQuote [114:121]
message StockPricePayload [123:126]
message IndicatorDataPoint [128:131]
message TechnicalIndicatorsPayload [133:139]
message CompanyOverviewPayload [141:153]
message InsiderTransaction [155:164]
message InsiderTransactionsPayload [166:168]
message NewsArticle [170:181]
message NewsSentimentPayload [183:185]
message AnnualEarning [187:190]
message QuarterlyEarning [192:199]
message EarningsHistoryPayload [201:204]
message EarningsCalendarEntry [206:213]
message EarningsCalendarPayload [215:217]
message AbnormalEventEntry [219:225]
message AbnormalEventPayload [227:229]
message TrendEntry [231:240]
message TrendPayload [242:244]
message ListSourcesRequest [248:248]
message SourceInfo [250:255]
message ListSourcesResponse [257:259]
message ListStrategiesRequest [263:263]
message StrategyStepInfo [265:269]
message StrategyInfo [271:275]
message ListStrategiesResponse [277:279]
message GetDashboardStatsRequest [283:283]
message SourceHealthSummary [285:288]
message EventTypeSummary [290:293]
message DashboardStats [295:303]
```

### C++ (grpc_rust_generator.cc, first 20)
```
namespace rust_grpc_generator [34:953]
  namespace <anonymous> [45:141]
    template_function GrpcGetCommentsForDescriptor [46:54]  -- std::string GrpcGetCommentsForDescriptor(const DescriptorType *descriptor)
    function RustModuleForContainingType [56:85]  -- std::string RustModuleForContainingType(const GrpcOpts &opts,
                                        const Descriptor *containing_type,
                                        const FileDescriptor &file)
    function RsTypePathWithinMessageModule [87:92]  -- std::string RsTypePathWithinMessageModule(const GrpcOpts &opts,
                                          const Descriptor &msg)
    function RsTypePath [94:115]  -- std::string RsTypePath(const Descriptor &msg, const GrpcOpts &opts, int depth)
    function ReadFileToString [117:140]  -- absl::Status ReadFileToString(const absl::string_view name, std::string *output,
                              bool text_mode)
  function GetImportPathToCrateNameMap [143:170]  -- absl::StatusOr<absl::flat_hash_map<std::string, std::string>> GetImportPathToCrateNameMap(const absl::string_view mapping_file_path)
  class Method [176:219]
    function Method [178:178]  -- Method() = delete;
    function Method [180:180]  -- explicit Method(const MethodDescriptor *method) : method_(method)
    function Name [183:185]  -- std::string Name() const
    function FullName [188:188]  -- absl::string_view FullName() const
    function ProtoFieldName [191:191]  -- absl::string_view ProtoFieldName() const
    function IsClientStreaming [194:194]  -- bool IsClientStreaming() const
    function IsServerStreaming [197:197]  -- bool IsServerStreaming() const
```
