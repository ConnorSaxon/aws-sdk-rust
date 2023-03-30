// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_documentation_part_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDocumentationPartInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.location {
        #[allow(unused_mut)]
        let mut object_2 = object.key("location").start_object();
        crate::protocol_serde::shape_documentation_part_location::ser_documentation_part_location(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.properties {
        object.key("properties").string(var_3.as_str());
    }
    Ok(())
}

