// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_game_server_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RegisterGameServerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.game_server_group_name {
        object.key("GameServerGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.game_server_id {
        object.key("GameServerId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_id {
        object.key("InstanceId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.connection_info {
        object.key("ConnectionInfo").string(var_4.as_str());
    }
    if let Some(var_5) = &input.game_server_data {
        object.key("GameServerData").string(var_5.as_str());
    }
    Ok(())
}

