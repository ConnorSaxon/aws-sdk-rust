// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_client_vpn_authentication_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ClientVpnAuthenticationRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Type");
    if let Some(var_2) = &input.r#type {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ActiveDirectory");
    if let Some(var_4) = &input.active_directory {
        crate::protocol_serde::shape_directory_service_authentication_request::ser_directory_service_authentication_request(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MutualAuthentication");
    if let Some(var_6) = &input.mutual_authentication {
        crate::protocol_serde::shape_certificate_authentication_request::ser_certificate_authentication_request(scope_5, var_6)?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("FederatedAuthentication");
    if let Some(var_8) = &input.federated_authentication {
        crate::protocol_serde::shape_federated_authentication_request::ser_federated_authentication_request(scope_7, var_8)?;
    }
    Ok(())
}

