// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cloud_front_origin_access_identity_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateCloudFrontOriginAccessIdentityOutput, crate::error::CreateCloudFrontOriginAccessIdentityError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateCloudFrontOriginAccessIdentityError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateCloudFrontOriginAccessIdentityError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CloudFrontOriginAccessIdentityAlreadyExists" => crate::error::CreateCloudFrontOriginAccessIdentityError::CloudFrontOriginAccessIdentityAlreadyExists({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_front_origin_access_identity_already_exists::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_front_origin_access_identity_already_exists::de_cloud_front_origin_access_identity_already_exists_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InconsistentQuantities" => crate::error::CreateCloudFrontOriginAccessIdentityError::InconsistentQuantities({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::inconsistent_quantities::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_inconsistent_quantities::de_inconsistent_quantities_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgument" => crate::error::CreateCloudFrontOriginAccessIdentityError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MissingBody" => crate::error::CreateCloudFrontOriginAccessIdentityError::MissingBody({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::missing_body::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_missing_body::de_missing_body_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyCloudFrontOriginAccessIdentities" => crate::error::CreateCloudFrontOriginAccessIdentityError::TooManyCloudFrontOriginAccessIdentities({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_cloud_front_origin_access_identities::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_cloud_front_origin_access_identities::de_too_many_cloud_front_origin_access_identities_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateCloudFrontOriginAccessIdentityError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cloud_front_origin_access_identity_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateCloudFrontOriginAccessIdentityOutput, crate::error::CreateCloudFrontOriginAccessIdentityError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_cloud_front_origin_access_identity_output::Builder::default();
        let _ = response;
        output = output.set_cloud_front_origin_access_identity(
            crate::protocol_serde::shape_create_cloud_front_origin_access_identity_output::de_cloud_front_origin_access_identity_payload(response.body().as_ref())?
        );
        output = output.set_e_tag(
            crate::protocol_serde::shape_create_cloud_front_origin_access_identity_output::de_e_tag_header(response.headers())
                                    .map_err(|_|crate::error::CreateCloudFrontOriginAccessIdentityError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output = output.set_location(
            crate::protocol_serde::shape_create_cloud_front_origin_access_identity_output::de_location_header(response.headers())
                                    .map_err(|_|crate::error::CreateCloudFrontOriginAccessIdentityError::unhandled("Failed to parse Location from header `Location"))?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

