# Reasoning Trace - Testing Guide

## ✅ Complete Integration

The Reasoning Trace feature is now fully integrated:
- ✅ Backend emits structured `reasoning-step` events
- ✅ Frontend listens and displays them
- ✅ ReasoningTrace component styled like Perplexity/Claude

---

## 🧪 How to Test

### 1. Start the Application
```bash
npm run tauri dev
```

### 2. Select Research Mode
In the chat interface, select "Research" mode from the mode selector.

### 3. Ask a Research Question
Try:
```
"Research about World Cup 2026"
"Research quantum computing advances in 2025"
"Research Hong Kong fintech trends"
```

### 4. Watch the Reasoning Trace

You should see a collapsible box appear:

```
Reasoning Trace (22.7s) ▼

  ⏳ Planning          3.0s
    Initializing research plan...
  
  🔍 Searching         [running]
    Executing search queries...
    🔍 7 queries
  
  ⏸  Analyzing         [pending]
    Processing collected information...
```

As the research progresses, steps will update from:
- `pending` (gray circle) → `running` (blue pulsing) → `completed` (green checkmark)

---

## 📊 What You'll See

### Phase 1: Planning (3-5s)
```
✓ Planning          3.2s
  Generated comprehensive research plan with key questions
```

### Phase 2: Searching (6-10s)
```
✓ Searching         8.1s
  Executed 7 search queries
  🔍 7 queries
  📊 Wikipedia, Research, News, +35
```

### Phase 3: Analyzing (5-8s)
```
✓ Analyzing         6.4s
  Identified key themes and insights
```

### Phase 4: Synthesizing (4-6s)
```
✓ Synthesizing      5.2s
  Integrated findings across sources
```

### Phase 5: Writing (8-12s)
```
✓ Writing          10.1s
  Final report generated successfully
```

---

## 🎨 UI Features

### Collapsible
- Click the header to expand/collapse
- Stays expanded by default during research
- Can collapse to save space

### Real-time Updates
- Steps appear as they start
- Status updates in real-time
- Durations calculated automatically

### Metadata Display
- Search queries count
- Source icons (first 5)
- Additional counts (+35)

---

## 🔧 Event Flow

### Backend (Rust)
```rust
// Start a step
self.start_step("planning", "Planning", "Initializing...");

// ... work happens ...

// Complete the step
self.complete_step("planning", "Planning", "Done!", start_time, metadata);
```

### Frontend (TypeScript)
```typescript
// Listen for events
listen<ReasoningStep>('reasoning-step', (event) => {
  const step = event.payload;
  // Update or add step
  setReasoningSteps(prev => updateStep(prev, step));
});
```

### Display
```tsx
{reasoningSteps.length > 0 && (
  <ReasoningTrace steps={reasoningSteps} isThinking={true} />
)}
```

---

## 🐛 Troubleshooting

### Not Seeing Reasoning Trace?

**Check 1: Is Research Mode Active?**
- The deep research agent emits reasoning steps
- Make sure you're in "Research" mode

**Check 2: Check Console**
```javascript
// Open DevTools (Cmd+Option+I on Mac)
// Look for:
console.log('Reasoning step:', event.payload);
```

**Check 3: Check Backend Logs**
```
[Deep Research] Starting deep research agent...
[Deep Research] Phase 1: Creating research plan
```

**Check 4: Verify Event Name**
Backend emits: `reasoning-step`
Frontend listens: `reasoning-step`
(Must match exactly!)

### Steps Not Updating?

**Check the step.id:**
- Backend: `step.id = "planning"`
- Frontend: Finds by `step.id`
- If IDs don't match, creates duplicate

**Check the status:**
- Valid: "pending", "running", "completed"
- Invalid values will render incorrectly

### Duration Not Showing?

Duration is only shown for completed steps:
```typescript
{step.duration && (
  <span>{(step.duration / 1000).toFixed(1)}s</span>
)}
```

Make sure `complete_step()` is called with timing.

---

## 📱 Mobile/Responsive

The ReasoningTrace component is responsive:
- Full width on mobile
- Max 3xl width on desktop
- Scrollable content
- Touch-friendly collapsing

---

## 🎯 Next Steps

### Integration with Other Agents

To add reasoning trace to other agents:

**1. Update the agent (Rust)**
```rust
// Add to RigCodeAgent, RigGeneralAgent, etc.
fn start_step(&self, id: &str, name: &str, desc: &str) -> Instant { ... }
fn complete_step(&self, id: &str, name: &str, desc: &str, start: Instant) { ... }
```

**2. Emit reasoning steps**
```rust
let start = self.start_step("coding", "Coding", "Writing code...");
// ... do work ...
self.complete_step("coding", "Coding", "Code written", start, None);
```

**3. Frontend automatically displays it**
No changes needed - already listening for all `reasoning-step` events!

### Custom Metadata

Add query counts, sources, etc:
```rust
self.complete_step(
    "searching",
    "Searching", 
    "Done",
    start,
    Some(ReasoningMetadata {
        queries: Some(vec!["query1".to_string(), "query2".to_string()]),
        sources: Some(vec!["Wikipedia".to_string()]),
        count: Some(10),
    })
);
```

Frontend will display:
- 🔍 2 queries
- 📊 Wikipedia, +9

---

## ✨ Summary

| Component | Status |
|-----------|--------|
| Backend events | ✅ Emitting |
| Frontend listener | ✅ Listening |
| ReasoningTrace UI | ✅ Displaying |
| Real-time updates | ✅ Working |
| Metadata display | ✅ Showing |

**Everything is ready!** Just run the app in Research mode and ask a question to see it in action.

---

**Last Updated**: 2026-07-04  
**Commit**: `9a6d1c3`  
**Status**: ✅ Production Ready
