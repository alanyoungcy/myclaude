# Deep Research Implementation

## Architecture

Based on LangChain's open_deep_research using Rust AutoAgents framework.

### Three-Phase Pipeline

```
┌─────────┐      ┌──────────┐      ┌───────┐
│  Scope  │  →   │ Research │  →   │ Write │
└─────────┘      └──────────┘      └───────┘
```

#### Phase 1: Scope
- **User Clarification**: Gather context to seed research
- **Brief Generation**: Create structured research brief

#### Phase 2: Research
- **Research Supervisor**: Coordinates the research process
- **Research Sub-agents**: Parallel agents researching specific topics
- **Web Search Tool**: Tavily API integration

#### Phase 3: Write
- **One-Shot Report Generation**: Synthesize findings into final report

## Implementation

### Key Components

1. **DeepResearchManager**
   - Orchestrates the three-phase pipeline
   - Manages AutoAgents lifecycle
   - Coordinates between agents

2. **Agents**
   - `BriefGeneratorAgent`: Generates research brief
   - `ResearchSupervisorAgent`: Supervises research
   - `ResearchSubAgent`: Executes specific research tasks
   - `ReportGeneratorAgent`: Generates final report

3. **Tools**
   - `WebSearchTool`: Web search via Tavily
   - `ClarifyTool`: User clarification

### Usage

```rust
let manager = DeepResearchManager::new(llm_wrapper, tavily_key);

// Full pipeline
let report = manager.run_deep_research("research topic").await?;

// Quick research
let findings = manager.run_simple_research("quick question").await?;
```

## Features

✅ Multi-agent coordination
✅ Parallel sub-agent research
✅ Web search integration
✅ Structured output
✅ Memory management
✅ Type-safe tool calling

## Reference

- LangChain: https://github.com/langchain-ai/open_deep_research
- AutoAgents: https://github.com/liquidos-ai/AutoAgents
