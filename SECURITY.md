# Security Policy

## API Key Security

### Important Notice

**A Mem0 API key was previously committed to this repository's history (commits prior to `1a595e4`).**

- **Key**: `m0-wXmQydXIdi1vZIbtcZ66xrpcyoJEusHyEiuY5SWG`
- **Status**: This key has been removed from the codebase but remains in Git history
- **Action Required**: This key should be considered compromised and revoked

### Current Security Measures

✅ All API keys removed from codebase (as of commit `1a595e4`)
✅ `.env` file is in `.gitignore`
✅ `.env.example` contains only placeholder values
✅ No hardcoded API keys in source code

### Best Practices

1. **Never commit API keys** to version control
2. **Use `.env` files** for local configuration (already in `.gitignore`)
3. **Use environment variables** for production deployments
4. **Rotate keys immediately** if accidentally committed
5. **Use separate keys** for development and production

### Getting Your Own API Keys

#### Required Keys:

1. **LLM Provider** (OpenAI, Anthropic, etc.)
   - Sign up at provider's website
   - Generate API key from dashboard

2. **Tavily API** (for web search)
   - Sign up at https://tavily.com
   - Free tier available
   - Get API key from dashboard

3. **Mem0 API** (for memory system)
   - Sign up at https://mem0.ai
   - Free tier available
   - Get API key from dashboard

### Configuration

Create a `.env` file in the project root:

```env
API_KEY=your_llm_api_key_here
BASE_URL=https://api.openai.com/v1
MODEL=gpt-4
TAVILY_API_KEY=your_tavily_key_here
MEM0_API_KEY=your_mem0_key_here
```

This file is automatically ignored by Git and will not be committed.

## Reporting Security Issues

If you discover a security vulnerability, please email the maintainer or open a private security advisory on GitHub.

**DO NOT** open a public issue for security vulnerabilities.

## History

- **2026-07-04**: Removed hardcoded Mem0 API key from codebase
- **2026-07-04**: Added this SECURITY.md document
