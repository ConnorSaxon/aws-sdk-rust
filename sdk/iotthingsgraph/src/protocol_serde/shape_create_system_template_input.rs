// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_system_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSystemTemplateInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.definition {
        #[allow(unused_mut)]
        let mut object_2 = object.key("definition").start_object();
        crate::protocol_serde::shape_definition_document::ser_definition_document(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.compatible_namespace_version {
        object.key("compatibleNamespaceVersion").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    Ok(())
}

