// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_file_system_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociateFileSystemInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.file_system_association_arn {
        object.key("FileSystemAssociationARN").string(var_1.as_str());
    }
    if input.force_delete {
        object.key("ForceDelete").boolean(input.force_delete);
    }
    Ok(())
}

