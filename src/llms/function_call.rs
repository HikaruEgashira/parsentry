use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Value,  // JSON Schema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,  // JSON string
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_function_serialization() {
        let function = Function {
            name: "get_weather".to_string(),
            description: "Get the weather in a given location".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    }
                },
                "required": ["location"]
            }),
        };

        let serialized = serde_json::to_string(&function).unwrap();
        let deserialized: Function = serde_json::from_str(&serialized).unwrap();
        assert_eq!(function.name, deserialized.name);
        assert_eq!(function.description, deserialized.description);
    }

    #[test]
    fn test_function_call_serialization() {
        let function_call = FunctionCall {
            name: "get_weather".to_string(),
            arguments: r#"{"location":"San Francisco, CA"}"#.to_string(),
        };

        let serialized = serde_json::to_string(&function_call).unwrap();
        let deserialized: FunctionCall = serde_json::from_str(&serialized).unwrap();
        assert_eq!(function_call.name, deserialized.name);
        assert_eq!(function_call.arguments, deserialized.arguments);
    }
}