// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_network_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateNetworkProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.network_profile_name {
        object.key("NetworkProfileName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ssid {
        object.key("Ssid").string(var_3.as_str());
    }
    if let Some(var_4) = &input.security_type {
        object.key("SecurityType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.eap_method {
        object.key("EapMethod").string(var_5.as_str());
    }
    if let Some(var_6) = &input.current_password {
        object.key("CurrentPassword").string(var_6.as_str());
    }
    if let Some(var_7) = &input.next_password {
        object.key("NextPassword").string(var_7.as_str());
    }
    if let Some(var_8) = &input.certificate_authority_arn {
        object.key("CertificateAuthorityArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.trust_anchors {
        let mut array_10 = object.key("TrustAnchors").start_array();
        for item_11 in var_9 {
             {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.tags {
        let mut array_14 = object.key("Tags").start_array();
        for item_15 in var_13 {
             {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    Ok(())
}

