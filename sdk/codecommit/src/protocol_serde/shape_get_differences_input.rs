// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_differences_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetDifferencesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.repository_name {
        object.key("repositoryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.before_commit_specifier {
        object.key("beforeCommitSpecifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.after_commit_specifier {
        object.key("afterCommitSpecifier").string(var_3.as_str());
    }
    if let Some(var_4) = &input.before_path {
        object.key("beforePath").string(var_4.as_str());
    }
    if let Some(var_5) = &input.after_path {
        object.key("afterPath").string(var_5.as_str());
    }
    if let Some(var_6) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    if let Some(var_7) = &input.next_token {
        object.key("NextToken").string(var_7.as_str());
    }
    Ok(())
}

