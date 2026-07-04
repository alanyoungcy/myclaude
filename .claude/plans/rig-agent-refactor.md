# Rig Framework Agent Refactoring Plan

## Overview
Refactor the current manual agent implementation to use the Rig framework for proper agent architecture with tool calling, memory integration, and extensibility.

## Current Problems

### 1. **Manual Tool Calling Loop**
- Current implementation manually manages tool calling in a loop
- No proper agent abstraction
- Hard to extend with new tools
- Error-prone tool execution logic

### 2. **Tight Coupling**
- Agents directly manage LLM client, Mem0 client, Tavily client
- Hard to test and mock
- Difficult to add new providers

### 3. **Inconsistent Tool Interface**
- Each agent reimplements tool definitions
- No type-safe tool arguments
- Manual JSON parsing everywhere

### 4. **Limited Extensibility**
- Adding new agents requires duplicating logic
- No way to compose agents
- Can't reuse tools across agents

## Rig Framework Benefits

### 1. **Built-in Agent System**
- `Agent<M>` provides proper agent abstraction
- Automatic tool calling loop with max iterations
- Built-in streaming support
- Hook system for observability

### 2. **Type-Safe Tools**
- `Tool` trait with proper error handling
- Automatic JSON schema generation
- Type-safe argument parsing via serde

### 3. **Provider Abstraction**
- `CompletionModel` trait for any LLM provider
- Built-in OpenAI, Anthropic, etc.
- Easy to add custom providers

### 4. **Composability**
- Agents can be used as tools
- Tool reuse across agents
- Memory integration via `Document` trait

## Implementation Plan

### Phase 1: Add Rig Dependencies & Custom Provider

**Goal**: Set up Rig with custom OpenAI-compatible provider

**Files to Create**:
1. `src/rig_provider.rs` - Custom OpenAI-compatible provider
   - Create `CustomOpenAIClient` that wraps our config
   - Implement `CompletionModel` for custom base URL support
   - Support dynamic base_url/api_key from config

**Files to Modify**:
1. `Cargo.toml` - Add rig-core, anyhow, thiserror
2. `src/lib.rs` - Export new rig_provider module

**Tests**:
- Test custom provider connection
- Test chat completion with custom base URL

### Phase 2: Implement Rig-Based Tools

**Goal**: Convert existing tools to Rig's Tool trait

**Files to Create**:
1. `src/tools/web_search.rs`
   - Implement `Tool` for `WebSearchTool`
   - Wrap TavilyClient
   - Proper error types with thiserror

2. `src/tools/calculator.rs`
   - Implement `Tool` for `CalculatorTool`
   - Use meval for expression evaluation
   - Handle math errors properly

3. `src/tools/file_ops.rs`
   - Implement `Tool` for `ReadFileTool`, `WriteFileTool`, `ListFilesTool`
   - Wrap tauri file operations
   - Proper path validation

4. `src/tools/command_executor.rs`
   - Implement `Tool` for `ExecuteCommandTool`
   - Wrap tauri command execution
   - Security checks

5. `src/tools/mod.rs`
   - Export all tools
   - Helper functions to box tools

**Tool Structure Example**:
```rust
use rig::tool::Tool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct WebSearchArgs {
    query: String,
    max_results: Option<u32>,
}

#[derive(Serialize)]
struct WebSearchTool {
    tavily_client: TavilyClient,
}

impl Tool for WebSearchTool {
    const NAME: &'static str = "web_search";
    type Error = WebSearchError;
    type Args = WebSearchArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        // Return JSON schema
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Execute search
    }
}
```

**Tests**:
- Unit test each tool independently
- Mock external dependencies (Tavily, file system)

### Phase 3: Refactor General Agent

**Goal**: Convert GeneralAgent to use Rig framework

**Files to Modify**:
1. `src/general_agent.rs`
   - Replace manual loop with Rig `Agent`
   - Use `.tools()` builder to add web_search, calculator
   - Integrate Mem0 via context retrieval before agent call
   - Emit progress via Rig hooks

**New Structure**:
```rust
pub struct GeneralAgent {
    rig_agent: Agent<CustomOpenAICompletionModel>,
    mem0_client: Option<Mem0Client>,
    app_handle: tauri::AppHandle,
    user_id: String,
}

impl GeneralAgent {
    pub fn new(...) -> Self {
        let provider = CustomOpenAIClient::new(base_url, api_key);
        let model = provider.completion_model("gpt-4");
        
        let rig_agent = model
            .agent()
            .preamble("You are a helpful general assistant...")
            .tool(WebSearchTool::new(tavily_key))
            .tool(CalculatorTool)
            .max_tokens(2048)
            .build();
            
        Self { rig_agent, mem0_client, ... }
    }
    
    pub async fn chat(&self, query: &str) -> Result<String> {
        // 1. Search Mem0 for context
        let context = self.search_memory(query).await;
        
        // 2. Build prompt with context
        let prompt = format!("{}\n\nContext: {}", query, context);
        
        // 3. Call Rig agent (handles tool loop automatically)
        let response = self.rig_agent.prompt(&prompt).await?;
        
        // 4. Save to Mem0
        self.save_memory(query, &response).await;
        
        Ok(response)
    }
}
```

**Tests**:
- Test general agent with mock provider
- Test Mem0 integration
- Test tool calling flow

### Phase 4: Refactor Code Agent

**Goal**: Convert CodeAgent to use Rig framework

**Files to Modify**:
1. `src/code_agent.rs`
   - Use Rig `Agent` with file operation tools
   - Add read_file, write_file, list_files, execute_command tools
   - Proper error handling with thiserror

**Tests**:
- Test code agent with mock file system
- Test command execution (sandboxed)

### Phase 5: Refactor Resume Agent

**Goal**: Convert ResumeAgent to use Rig framework

**Files to Modify**:
1. `src/resume_agent.rs`
   - Use Rig `Agent` with no external tools (pure LLM)
   - Focus on prompt engineering
   - Mem0 integration for candidate profiles

**Tests**:
- Test resume generation
- Test job analysis

### Phase 6: Refactor Research Agent

**Goal**: Convert deep research to use Rig agents

**Files to Modify**:
1. `src/deep_research.rs`
   - Use Rig `Agent` for each phase
   - Web search tool for gathering phase
   - Agent composition: planner → searcher → writer

**Tests**:
- Test research pipeline
- Test phase transitions

### Phase 7: Integration & Commands

**Goal**: Wire up Rig agents to Tauri commands

**Files to Modify**:
1. `src/commands.rs`
   - Replace agent instantiation with Rig agents
   - Keep same command signatures
   - Handle streaming via Rig's streaming support

**Tests**:
- Integration tests with all modes
- Test command error handling

### Phase 8: Cleanup & Documentation

**Goal**: Remove old code, add documentation

**Files to Delete**:
- Remove old `src/llm.rs` if fully replaced
- Clean up unused dependencies (autoagents if not needed)

**Files to Update**:
1. `README.md`
   - Document Rig-based architecture
   - Update architecture diagrams
   - Add tool development guide

2. Create `docs/AGENT_DEVELOPMENT.md`
   - How to add new tools
   - How to create new agents
   - Testing guide

**Tests**:
- Full end-to-end tests
- Performance benchmarks

## Key Design Decisions

### 1. Custom Provider vs Built-in
**Decision**: Create custom OpenAI-compatible provider
**Reason**: Need to support any OpenAI-compatible API with custom base_url

### 2. Tool Organization
**Decision**: Separate `tools/` module with one file per tool
**Reason**: Better organization, easier to test, reusable across agents

### 3. Mem0 Integration
**Decision**: Keep Mem0 outside Rig agent, call before/after
**Reason**: Mem0 is not a RAG system, it's contextual memory

### 4. Streaming
**Decision**: Use Rig's streaming support where available
**Reason**: Better UX, Rig has built-in streaming

### 5. Error Handling
**Decision**: Use anyhow for application errors, thiserror for tool errors
**Reason**: Rig uses Result<T, E> where E: Error, thiserror provides good Error impls

## Testing Strategy

### Unit Tests
- Each tool independently
- Mock external dependencies
- Test error cases

### Integration Tests
- Full agent flows
- Multi-turn conversations
- Tool calling sequences

### Manual Testing Checklist
1. General mode: web search works
2. General mode: calculator works
3. Code mode: file operations work
4. Code mode: command execution works
5. Research mode: full pipeline works
6. Resume mode: generation works
7. Mem0: context is retrieved and saved
8. Streaming: real-time responses
9. Error handling: graceful failures

## Migration Path

### Step 1: Parallel Implementation
- Keep old agents working
- Implement new Rig agents alongside
- Add feature flag `use_rig_agents`

### Step 2: Testing
- Test new agents thoroughly
- Compare outputs with old agents
- Performance testing

### Step 3: Switch
- Enable `use_rig_agents` by default
- Keep old code for one release

### Step 4: Cleanup
- Remove old agent code
- Remove feature flag
- Update all documentation

## Risk Mitigation

### Risk 1: Rig API Changes
**Mitigation**: Pin rig-core version, test before upgrades

### Risk 2: Performance Regression
**Mitigation**: Benchmark before/after, optimize hot paths

### Risk 3: Breaking Changes
**Mitigation**: Keep same Tauri command interface, changes are internal

### Risk 4: Tool Compatibility
**Mitigation**: Comprehensive tool tests, validate schemas

## Success Criteria

1. ✅ All agents use Rig framework
2. ✅ All tools are type-safe with Tool trait
3. ✅ All existing functionality works
4. ✅ All tests pass
5. ✅ Code is more maintainable (fewer lines, clearer structure)
6. ✅ Easy to add new tools and agents
7. ✅ Documentation is updated
8. ✅ No performance regression

## Timeline Estimate

- Phase 1: 1-2 hours (setup, custom provider)
- Phase 2: 2-3 hours (5 tools)
- Phase 3: 1 hour (general agent)
- Phase 4: 1 hour (code agent)
- Phase 5: 30 min (resume agent)
- Phase 6: 1 hour (research agent)
- Phase 7: 1 hour (integration)
- Phase 8: 1 hour (cleanup, docs)

**Total: 8-10 hours**

## Next Steps

1. Get user approval for this plan
2. Start with Phase 1: Setup and custom provider
3. Implement Phase 2: Tools (can be done in parallel)
4. Proceed through phases sequentially
5. Test thoroughly at each phase
