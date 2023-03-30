// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_evaluate_code_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::EvaluateCodeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.code {
        object.key("code").string(var_1.as_str());
    }
    if let Some(var_2) = &input.context {
        object.key("context").string(var_2.as_str());
    }
    if let Some(var_3) = &input.function {
        object.key("function").string(var_3.as_str());
    }
    if let Some(var_4) = &input.runtime {
        #[allow(unused_mut)]
        let mut object_5 = object.key("runtime").start_object();
        crate::protocol_serde::shape_app_sync_runtime::ser_app_sync_runtime(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

