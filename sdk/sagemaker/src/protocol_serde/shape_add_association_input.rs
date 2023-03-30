// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_association_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddAssociationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.source_arn {
        object.key("SourceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.destination_arn {
        object.key("DestinationArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.association_type {
        object.key("AssociationType").string(var_3.as_str());
    }
    Ok(())
}

