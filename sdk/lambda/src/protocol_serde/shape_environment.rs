// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_environment(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Environment) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.variables {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Variables").start_object();
        for (key_3, value_4) in var_1 {
             {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    Ok(())
}

