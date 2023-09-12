// Module: json_validator
use serde_json::Value;

// add return value to function
pub fn validate_json(data: Value) -> Result<String, &'static str> {
    let bytecode = data.get("bytecode");

    if let Some(bytecode) = bytecode {
        let object = bytecode.get("object");

        if let Some(object) = object {
            return Ok(object.as_str().unwrap().to_string().to_string());
        } else {
            return Err("The child key `object` does not exist");
        }
    } else {
        return Err("The key `bytecode` does not exist");
    }
}