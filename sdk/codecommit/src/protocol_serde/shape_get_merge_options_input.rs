// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_merge_options_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetMergeOptionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.repository_name {
        object.key("repositoryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.source_commit_specifier {
        object.key("sourceCommitSpecifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.destination_commit_specifier {
        object.key("destinationCommitSpecifier").string(var_3.as_str());
    }
    if let Some(var_4) = &input.conflict_detail_level {
        object.key("conflictDetailLevel").string(var_4.as_str());
    }
    if let Some(var_5) = &input.conflict_resolution_strategy {
        object.key("conflictResolutionStrategy").string(var_5.as_str());
    }
    Ok(())
}

