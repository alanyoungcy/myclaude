# 🔧 Tool Calling System - Web Search

## ✅ Implementation Complete

Web search is now implemented as an **automatic tool** that Claude can call when needed, not a separate button.

## How It Works

### User Experience:
1. User sends a message normally (one Send button)
2. Claude decides if web search is needed
3. If needed, Claude automatically calls the `web_search` tool
4. Search results are retrieved from Tavily
5. Claude uses the results to answer the question
6. User sees the final answer (with search context)

### Example Flow:

**User**: "What's the weather in Tokyo today?"

**Behind the scenes**:
1. Claude receives: system prompt, conversation history, user message
2. Claude sees available tool: `web_search`
3. Claude decides: "I need current weather info" 
4. Claude calls: `web_search(query: "weather Tokyo today")`
5. System executes: Tavily API search
6. System returns: 5 relevant results with weather data
7. Claude receives: Search results
8. Claude responds: "Based on current data, Tokyo is..."

**User sees**: Just Claude's final answer with current information!

## Technical Architecture

### 1. Tool Definition (Rust)
```rust
Tool {
    type: "function",
    function: {
        name: "web_search",
        description: "Search the web for current information...",
        parameters: {
            type: "object",
            properties: {
                query: {
                    type: "string",
                    description: "The search query..."
                }
            },
            required: ["query"]
        }
    }
}
```

### 2. LLM Request with Tools
```rust
ChatRequest {
    model: "claude-fable-5",
    messages: [...],
    tools: Some([web_search_tool]),  // ← Tool available
    stream: None
}
```

### 3. Tool Call Detection
When Claude responds with `tool_calls`:
```rust
{
    "tool_calls": [{
        "id": "call_abc123",
        "type": "function",
        "function": {
            "name": "web_search",
            "arguments": "{\"query\":\"weather Tokyo\"}"
        }
    }]
}
```

### 4. Tool Execution
System executes the tool:
```rust
let query = parse_arguments(tool_call.arguments);
let results = tavily_client.search(query, 5).await;
```

### 5. Tool Response
Return results to Claude:
```rust
Message {
    role: "tool",
    tool_call_id: "call_abc123",
    name: "web_search",
    content: "Search results for 'weather Tokyo':\n\n1. ..."
}
```

### 6. Final Response
Claude generates final answer with search context:
```rust
Message {
    role: "assistant",
    content: "Based on current weather data from Tokyo..."
}
```

## When Claude Uses Web Search

Claude automatically calls web search for:

✅ **Current Events**: "What happened in the news today?"
✅ **Weather**: "What's the weather in Paris?"
✅ **Recent Facts**: "Who won the latest Super Bowl?"
✅ **Real-time Data**: "What's the current stock price of Apple?"
✅ **Verification**: "Is it true that X happened?"
✅ **Research**: "What are the latest developments in quantum computing?"

Claude does NOT call web search for:

❌ **General Knowledge**: "What is Python?" (already knows)
❌ **Math**: "What is 2+2?" (can calculate)
❌ **Definitions**: "Define machine learning" (in knowledge base)
❌ **Code**: "Write a function to sort an array" (can generate)

## Code Structure

### Message Flow:
```
User Input
    ↓
[Save to DB]
    ↓
[Build message history]
    ↓
[Add tool definition if Tavily key exists]
    ↓
[Call LLM with tools]
    ↓
[Check response for tool_calls]
    ↓
If tool_calls present:
    ├─ Parse tool call arguments
    ├─ Execute web_search via Tavily
    ├─ Format results
    ├─ Add tool response to messages
    ├─ Call LLM again with tool results
    └─ Get final answer
    ↓
[Save assistant response to DB]
    ↓
Return to User
```

### Key Components:

**llm.rs**:
- Updated `Message` struct with optional fields
- Added `ToolCall`, `FunctionCall` structures
- Support for `tool_calls`, `tool_call_id`, `name` fields

**commands.rs**:
- Tool definition with description and JSON schema
- Tool call detection logic
- Tool execution (Tavily search)
- Multi-turn conversation with tool results

**tavily.rs**:
- Tavily API client
- Search result formatting
- Error handling

## Configuration

### Required:
- `TAVILY_API_KEY` in `.env` (already set)
- Claude-compatible model that supports tool calling

### Tool Availability:
- Tool is only added if `tavily_api_key` is configured
- If not configured, chat works normally without tools

## Logging

When tools are called, you'll see:
```
Tool calls detected: 1 calls
Executing web_search tool call: call_abc123
Tavily search: weather Tokyo (max results: 5)
Tavily returned 5 results
Calling LLM with tool results
```

## Testing

Try these prompts:

1. **Direct question**: "What's the weather in London?"
   - Should trigger tool call automatically

2. **News**: "What are the top tech news stories today?"
   - Should search for current news

3. **General knowledge**: "What is Python?"
   - Should NOT trigger tool (Claude knows this)

4. **Comparison**: "Compare recent AI developments"
   - Should search for current info

## UI/UX

### What Users See:
- ✅ Single "Send" button
- ✅ Normal typing experience
- ✅ Loading indicator while processing
- ✅ Final answer with context (no tool internals shown)

### What Users DON'T See:
- ❌ Tool calls happening
- ❌ Search being executed
- ❌ Raw search results
- ❌ Multiple LLM calls

The tool calling is **completely transparent** to the user!

## Benefits

1. **Automatic**: Claude decides when to search
2. **Contextual**: Only searches when needed
3. **Seamless**: No UI changes for users
4. **Efficient**: Single button, smart backend
5. **Extensible**: Easy to add more tools later

## Future Tool Ideas

Using the same pattern, you could add:
- `code_execution` - Run code snippets
- `calculator` - Complex calculations
- `image_generation` - Create images
- `file_read` - Read file contents
- `database_query` - Query databases

## Architecture Advantages

✅ **Clean separation**: UI doesn't know about tools
✅ **LLM decides**: Claude determines when to use tools
✅ **Extensible**: Add new tools without UI changes
✅ **Standard**: Uses OpenAI tool calling format
✅ **Reliable**: Handles errors gracefully

---

**Status**: ✅ Implemented and Running
**Test it**: Ask Claude about current events or weather!
