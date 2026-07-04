# Memory System Explanation

## Two Types of Memory

MyClaude now has **two independent memory systems** working together:

### 1. 🧠 Conversation History (Chat History) - **Short-term Memory**
**Status**: ✅ **WORKING** (Fixed in latest update)

**What it does**:
- Remembers everything within the **current conversation session**
- Stored in RAM (`chat_history: Mutex<Vec<Message>>`)
- Automatically maintained by Rig's `Chat` trait
- Resets when you start a new conversation or restart the app

**Example**:
```
You: "My favorite color is blue"
Agent: "I'll remember that!"
You: "What's my favorite color?"
Agent: "Your favorite color is blue" ✅ (from chat history)
```

**How it works**:
```rust
pub struct RigGeneralAgent {
    chat_history: Mutex<Vec<Message>>,  // ✅ Stores all messages
}

// Each call adds to history
agent.chat(&query, &mut history).await?
```

### 2. 💾 Mem0 - **Long-term Memory** (Optional)
**Status**: ⚠️ **OPTIONAL** - Requires API key

**What it does**:
- Remembers facts **across different conversations**
- Stored in Mem0's cloud database
- Survives app restarts
- Retrieved based on semantic similarity

**Example**:
```
[Session 1]
You: "I'm a software engineer at Google"
Agent: "Got it!"

[App restart - Session 2]
You: "Where do I work?"
Agent: "You work at Google as a software engineer" ✅ (from Mem0)
```

**How it works**:
```rust
// Before answering, search Mem0
let context = self.search_memory(query).await;

// After answering, save to Mem0
self.save_memory(query, response).await;
```

---

## Current Status

### ✅ What's Working
1. **Conversation History** (Short-term)
   - Agents remember context within the same conversation
   - Multi-turn conversations work perfectly
   - No configuration needed

### ⚠️ What Needs Configuration
2. **Mem0** (Long-term)
   - Currently: `Mem0: EMPTY` (not configured)
   - Agents still work without it
   - You just lose cross-session memory

---

## How to Enable Mem0 (Optional)

If you want **long-term memory** across conversations:

### Step 1: Get a Mem0 API Key
1. Go to https://mem0.ai
2. Sign up for a free account
3. Get your API key

### Step 2: Configure Your .env File
Add this line to your `.env` file:
```bash
MEM0_API_KEY=your_mem0_api_key_here
```

### Step 3: Restart the App
Your configuration will show:
```
Config loaded - ... Mem0: SET
```

---

## Memory Behavior Comparison

### Without Mem0 (Current Setup)
| Scenario | Works? | Explanation |
|----------|--------|-------------|
| Same conversation | ✅ Yes | Chat history |
| After restart | ❌ No | No long-term memory |
| Different conversation | ❌ No | No cross-session memory |

### With Mem0 Configured
| Scenario | Works? | Explanation |
|----------|--------|-------------|
| Same conversation | ✅ Yes | Chat history |
| After restart | ✅ Yes | Mem0 database |
| Different conversation | ✅ Yes | Mem0 semantic search |

---

## Practical Examples

### Example 1: Without Mem0 (Current)
```
Conversation 1:
You: "I'm working on a React project"
Agent: "Great! What do you need help with?"
You: "Can you help me with useState?"
Agent: "Sure, here's how useState works..." ✅ (remembers React context)

[New conversation or restart]
You: "Continue helping with my project"
Agent: "What project are you working on?" ❌ (forgot about React)
```

### Example 2: With Mem0 Enabled
```
Conversation 1:
You: "I'm working on a React project"
Agent: "Great! What do you need help with?"

[New conversation or restart]
You: "Continue helping with my project"
Agent: "Sure, let's continue with your React project" ✅ (Mem0 remembered)
```

---

## Technical Details

### Conversation History Implementation
```rust
impl RigGeneralAgent {
    pub async fn chat(&self, query: &str) -> Result<String> {
        // Get mutable access to history
        let mut history = self.chat_history.lock().unwrap();
        
        // Rig automatically manages the conversation
        let response = agent.chat(&query, &mut *history).await?;
        
        // History now contains:
        // - Previous user messages
        // - Previous assistant responses
        // - Tool calls and results
        
        Ok(response)
    }
}
```

### Mem0 Integration (When Configured)
```rust
pub async fn chat(&self, query: &str) -> Result<String> {
    // 1. Search Mem0 for relevant context
    let context = self.search_memory(query).await;
    
    // 2. Add context to the conversation if found
    let enriched_query = if let Some(ctx) = context {
        format!("Context: {}\n\nQuery: {}", ctx.join("\n"), query)
    } else {
        query.to_string()
    };
    
    // 3. Chat with history + Mem0 context
    let mut history = self.chat_history.lock().unwrap();
    let response = agent.chat(&enriched_query, &mut *history).await?;
    
    // 4. Save interaction to Mem0 for future sessions
    self.save_memory(query, &response).await;
    
    Ok(response)
}
```

---

## Recommendation

### For Most Users
**You don't need Mem0 right now!**

The conversation history (chat_history) already solves the main problem you reported:
- ✅ Agents remember context within conversations
- ✅ Multi-turn discussions work perfectly
- ✅ No configuration needed

### When to Enable Mem0
Enable Mem0 if you want:
- Cross-session memory (remember things after restart)
- Profile persistence (career details, preferences, projects)
- Long-term context accumulation
- Semantic memory search

---

## Verification

To verify conversation history is working:

### Test 1: Basic Memory
```
You: "My name is Alice"
Agent: "Nice to meet you, Alice!"
You: "What's my name?"
Agent: "Your name is Alice" ✅
```

### Test 2: Context Continuity
```
You: "I'm building a web app with React"
Agent: "Great! What feature are you working on?"
You: "I need help with the login form"
Agent: "For your React app, here's how to build a login form..." ✅
```

### Test 3: Tool Use Memory
```
You: "Search for Python tutorials"
Agent: [uses web_search tool] "Here are some tutorials..."
You: "Which one looks best?"
Agent: "Based on the search results I just found..." ✅ (remembers the search)
```

---

## Summary

| Feature | Status | Required |
|---------|--------|----------|
| **Conversation History** | ✅ Working | Built-in |
| **Mem0 Long-term Memory** | ⚠️ Optional | API key needed |

**Bottom line**: Your main issue (agents not remembering within conversations) is **already fixed** with conversation history. Mem0 is just a nice-to-have for cross-session memory.

---

## Troubleshooting

### "Agent still doesn't remember things"
1. Make sure you're in the **same conversation**
2. Check that you didn't restart the app (conversation history is in RAM)
3. Verify you're using the new Rig agents (not old ones)

### "I want cross-session memory"
1. Get a Mem0 API key from https://mem0.ai
2. Add `MEM0_API_KEY=...` to your `.env` file
3. Restart the app
4. Look for "Mem0: SET" in the startup logs

### "Mem0 API errors"
If you configure Mem0 but get errors:
- Check your API key is correct
- Verify you have internet connection
- Mem0 has a free tier with usage limits
- Agents will still work without Mem0 (graceful fallback)

---

**Last Updated**: 2026-07-04  
**Applies to**: All Rig-based agents (RigGeneralAgent, RigCodeAgent, RigDeepResearchAgent, RigResumeAgent)
