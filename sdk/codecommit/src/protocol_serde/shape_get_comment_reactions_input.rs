// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_comment_reactions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetCommentReactionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.comment_id {
        object.key("commentId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.reaction_user_arn {
        object.key("reactionUserArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("nextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    Ok(())
}

