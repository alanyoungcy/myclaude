//! Calculator Tool for mathematical expressions
//!
//! This tool provides mathematical calculation capabilities using the meval library.

use rig_core::{
    completion::ToolDefinition,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

/// Arguments for calculator tool
#[derive(Debug, Deserialize)]
pub struct CalculatorArgs {
    /// The mathematical expression to evaluate (e.g., "2 + 2", "15% of 200", "sqrt(16)")
    pub expression: String,
}

/// Calculator tool errors
#[derive(Debug, Error)]
pub enum CalculatorError {
    #[error("Invalid expression: {0}")]
    InvalidExpression(String),
    #[error("Evaluation error: {0}")]
    EvaluationError(String),
}

/// Calculator Tool
///
/// Performs mathematical calculations using standard mathematical notation.
///
/// Supports:
/// - Basic arithmetic: +, -, *, /
/// - Exponentiation: ^
/// - Functions: sqrt, sin, cos, tan, ln, log, abs, etc.
/// - Constants: pi, e
///
/// # Example
/// ```no_run
/// use myclaude::tools::calculator::CalculatorTool;
///
/// let tool = CalculatorTool;
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct CalculatorTool;

impl Tool for CalculatorTool {
    const NAME: &'static str = "calculator";

    type Error = CalculatorError;
    type Args = CalculatorArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Perform mathematical calculations. Supports basic arithmetic (+, -, *, /), exponentiation (^), functions (sqrt, sin, cos, tan, ln, log, abs), and constants (pi, e). Use this when you need to calculate numerical results.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "Mathematical expression to evaluate (e.g., '2 + 2', '15 * 100 / 200', 'sqrt(16)', 'pi * 2')"
                    }
                },
                "required": ["expression"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Validate expression
        if args.expression.trim().is_empty() {
            return Err(CalculatorError::InvalidExpression(
                "Expression cannot be empty".to_string(),
            ));
        }

        // Parse and evaluate the expression
        let result = meval::eval_str(&args.expression)
            .map_err(|e| CalculatorError::EvaluationError(e.to_string()))?;

        // Format the result
        Ok(format!("{}", result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_arithmetic() {
        let tool = CalculatorTool;

        // Addition
        let result = tool
            .call(CalculatorArgs {
                expression: "2 + 2".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(result, "4");

        // Multiplication
        let result = tool
            .call(CalculatorArgs {
                expression: "5 * 3".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(result, "15");

        // Division
        let result = tool
            .call(CalculatorArgs {
                expression: "10 / 2".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(result, "5");
    }

    #[tokio::test]
    async fn test_complex_expression() {
        let tool = CalculatorTool;

        let result = tool
            .call(CalculatorArgs {
                expression: "(2 + 3) * 4".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(result, "20");
    }

    #[tokio::test]
    async fn test_functions() {
        let tool = CalculatorTool;

        // Square root
        let result = tool
            .call(CalculatorArgs {
                expression: "sqrt(16)".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(result, "4");

        // Absolute value
        let result = tool
            .call(CalculatorArgs {
                expression: "abs(-5)".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(result, "5");
    }

    #[tokio::test]
    async fn test_constants() {
        let tool = CalculatorTool;

        // Pi
        let result = tool
            .call(CalculatorArgs {
                expression: "pi".to_string(),
            })
            .await
            .unwrap();
        let pi_value: f64 = result.parse().unwrap();
        assert!((pi_value - std::f64::consts::PI).abs() < 0.0001);
    }

    #[tokio::test]
    async fn test_empty_expression() {
        let tool = CalculatorTool;

        let result = tool
            .call(CalculatorArgs {
                expression: "".to_string(),
            })
            .await;

        assert!(matches!(result, Err(CalculatorError::InvalidExpression(_))));
    }

    #[tokio::test]
    async fn test_invalid_expression() {
        let tool = CalculatorTool;

        let result = tool
            .call(CalculatorArgs {
                expression: "invalid + + expression".to_string(),
            })
            .await;

        assert!(result.is_err());
    }
}
