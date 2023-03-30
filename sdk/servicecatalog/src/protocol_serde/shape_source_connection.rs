// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_source_connection(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SourceConnection) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.r#type {
        object.key("Type").string(var_1.as_str());
    }
    if let Some(var_2) = &input.connection_parameters {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ConnectionParameters").start_object();
        crate::protocol_serde::shape_source_connection_parameters::ser_source_connection_parameters(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

