// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_comment_reaction_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutCommentReactionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.comment_id {
        object.key("commentId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.reaction_value {
        object.key("reactionValue").string(var_2.as_str());
    }
    Ok(())
}

