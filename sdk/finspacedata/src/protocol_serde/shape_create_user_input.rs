// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_user_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateUserInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.api_access {
        object.key("ApiAccess").string(var_1.as_str());
    }
    if let Some(var_2) = &input.api_access_principal_arn {
        object.key("apiAccessPrincipalArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.client_token {
        object.key("clientToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.email_address {
        object.key("emailAddress").string(var_4.as_str());
    }
    if let Some(var_5) = &input.first_name {
        object.key("firstName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.last_name {
        object.key("lastName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.r#type {
        object.key("type").string(var_7.as_str());
    }
    Ok(())
}

