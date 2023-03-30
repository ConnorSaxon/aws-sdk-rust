// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_custom_key_store_input(input: &crate::input::UpdateCustomKeyStoreInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_custom_key_store_input::ser_update_custom_key_store_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_custom_key_store_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateCustomKeyStoreOutput, crate::error::UpdateCustomKeyStoreError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateCustomKeyStoreError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CloudHsmClusterInvalidConfigurationException" => crate::error::UpdateCustomKeyStoreError::CloudHsmClusterInvalidConfigurationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_cluster_invalid_configuration_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_cluster_invalid_configuration_exception::de_cloud_hsm_cluster_invalid_configuration_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmClusterNotActiveException" => crate::error::UpdateCustomKeyStoreError::CloudHsmClusterNotActiveException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_cluster_not_active_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_cluster_not_active_exception::de_cloud_hsm_cluster_not_active_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmClusterNotFoundException" => crate::error::UpdateCustomKeyStoreError::CloudHsmClusterNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_cluster_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_cluster_not_found_exception::de_cloud_hsm_cluster_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CloudHsmClusterNotRelatedException" => crate::error::UpdateCustomKeyStoreError::CloudHsmClusterNotRelatedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_hsm_cluster_not_related_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_hsm_cluster_not_related_exception::de_cloud_hsm_cluster_not_related_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CustomKeyStoreInvalidStateException" => crate::error::UpdateCustomKeyStoreError::CustomKeyStoreInvalidStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::custom_key_store_invalid_state_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CustomKeyStoreNameInUseException" => crate::error::UpdateCustomKeyStoreError::CustomKeyStoreNameInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::custom_key_store_name_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_custom_key_store_name_in_use_exception::de_custom_key_store_name_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CustomKeyStoreNotFoundException" => crate::error::UpdateCustomKeyStoreError::CustomKeyStoreNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::custom_key_store_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSInternalException" => crate::error::UpdateCustomKeyStoreError::KmsInternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::kms_internal_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksProxyIncorrectAuthenticationCredentialException" => crate::error::UpdateCustomKeyStoreError::XksProxyIncorrectAuthenticationCredentialException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::xks_proxy_incorrect_authentication_credential_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_proxy_incorrect_authentication_credential_exception::de_xks_proxy_incorrect_authentication_credential_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksProxyInvalidConfigurationException" => crate::error::UpdateCustomKeyStoreError::XksProxyInvalidConfigurationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::xks_proxy_invalid_configuration_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_proxy_invalid_configuration_exception::de_xks_proxy_invalid_configuration_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksProxyInvalidResponseException" => crate::error::UpdateCustomKeyStoreError::XksProxyInvalidResponseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::xks_proxy_invalid_response_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_proxy_invalid_response_exception::de_xks_proxy_invalid_response_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksProxyUriEndpointInUseException" => crate::error::UpdateCustomKeyStoreError::XksProxyUriEndpointInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::xks_proxy_uri_endpoint_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_proxy_uri_endpoint_in_use_exception::de_xks_proxy_uri_endpoint_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksProxyUriInUseException" => crate::error::UpdateCustomKeyStoreError::XksProxyUriInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::xks_proxy_uri_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_proxy_uri_in_use_exception::de_xks_proxy_uri_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksProxyUriUnreachableException" => crate::error::UpdateCustomKeyStoreError::XksProxyUriUnreachableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::xks_proxy_uri_unreachable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_proxy_uri_unreachable_exception::de_xks_proxy_uri_unreachable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksProxyVpcEndpointServiceInUseException" => crate::error::UpdateCustomKeyStoreError::XksProxyVpcEndpointServiceInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::xks_proxy_vpc_endpoint_service_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_proxy_vpc_endpoint_service_in_use_exception::de_xks_proxy_vpc_endpoint_service_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksProxyVpcEndpointServiceInvalidConfigurationException" => crate::error::UpdateCustomKeyStoreError::XksProxyVpcEndpointServiceInvalidConfigurationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::xks_proxy_vpc_endpoint_service_invalid_configuration_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_proxy_vpc_endpoint_service_invalid_configuration_exception::de_xks_proxy_vpc_endpoint_service_invalid_configuration_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "XksProxyVpcEndpointServiceNotFoundException" => crate::error::UpdateCustomKeyStoreError::XksProxyVpcEndpointServiceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::xks_proxy_vpc_endpoint_service_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_xks_proxy_vpc_endpoint_service_not_found_exception::de_xks_proxy_vpc_endpoint_service_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateCustomKeyStoreError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_custom_key_store_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateCustomKeyStoreOutput, crate::error::UpdateCustomKeyStoreError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_custom_key_store_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

