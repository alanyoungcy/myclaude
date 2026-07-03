---
name: web_search
description: Search the web for current information, news, facts, or any information not in your knowledge base
parameters:
  - name: query
    type: string
    description: The search query to find relevant information
    required: true
---

# Web Search Skill

Search the web for up-to-date information when:
- User asks about current events or recent news
- Information might have changed since your knowledge cutoff
- You need to verify facts or find specific data
- User explicitly asks you to search

Return results with:
- Title and URL of each source
- Relevant excerpts
- Relevance score if available

Always cite your sources with URLs.
