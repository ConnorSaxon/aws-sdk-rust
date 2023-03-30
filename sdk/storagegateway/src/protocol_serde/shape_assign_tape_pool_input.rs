// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_assign_tape_pool_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssignTapePoolInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.tape_arn {
        object.key("TapeARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.pool_id {
        object.key("PoolId").string(var_2.as_str());
    }
    if input.bypass_governance_retention {
        object.key("BypassGovernanceRetention").boolean(input.bypass_governance_retention);
    }
    Ok(())
}

