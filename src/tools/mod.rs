//! Rig-based tools for agents
//!
//! This module provides type-safe tools that implement Rig's Tool trait.
//! Each tool handles its own argument parsing, validation, and execution.

pub mod web_search;
pub mod calculator;

// Re-export tools for convenience
pub use web_search::WebSearchTool;
pub use calculator::CalculatorTool;

use rig_core::tool::ToolDyn;

/// Create a boxed web search tool
///
/// # Arguments
/// * `api_key` - Optional Tavily API key
///
/// # Example
/// ```no_run
/// use myclaude::tools::boxed_web_search;
///
/// let tool = boxed_web_search(Some("tvly-...".to_string()));
/// ```
pub fn boxed_web_search(api_key: Option<String>) -> Box<dyn ToolDyn> {
    Box::new(WebSearchTool::new(api_key))
}

/// Create a boxed calculator tool
///
/// # Example
/// ```no_run
/// use myclaude::tools::boxed_calculator;
///
/// let tool = boxed_calculator();
/// ```
pub fn boxed_calculator() -> Box<dyn ToolDyn> {
    Box::new(CalculatorTool)
}

/// Create all basic tools (web search + calculator)
///
/// # Arguments
/// * `tavily_api_key` - Optional Tavily API key for web search
///
/// # Example
/// ```no_run
/// use myclaude::tools::all_basic_tools;
///
/// let tools = all_basic_tools(Some("tvly-...".to_string()));
/// ```
pub fn all_basic_tools(tavily_api_key: Option<String>) -> Vec<Box<dyn ToolDyn>> {
    let mut tools: Vec<Box<dyn ToolDyn>> = vec![boxed_calculator()];

    // Only add web search if API key is provided
    if tavily_api_key.is_some() {
        tools.push(boxed_web_search(tavily_api_key));
    }

    tools
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxed_web_search() {
        let tool = boxed_web_search(Some("test-key".to_string()));
        // Just verify it's a valid boxed tool
        assert!(tool.name() == "web_search");
    }

    #[test]
    fn test_boxed_calculator() {
        let tool = boxed_calculator();
        // Just verify it's a valid boxed tool
        assert!(tool.name() == "calculator");
    }

    #[test]
    fn test_all_basic_tools_with_key() {
        let tools = all_basic_tools(Some("test-key".to_string()));
        assert_eq!(tools.len(), 2); // calculator + web search
    }

    #[test]
    fn test_all_basic_tools_without_key() {
        let tools = all_basic_tools(None);
        assert_eq!(tools.len(), 1); // only calculator
    }
}
