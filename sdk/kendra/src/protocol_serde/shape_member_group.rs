// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_member_group(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MemberGroup) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.group_id {
        object.key("GroupId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.data_source_id {
        object.key("DataSourceId").string(var_2.as_str());
    }
    Ok(())
}

