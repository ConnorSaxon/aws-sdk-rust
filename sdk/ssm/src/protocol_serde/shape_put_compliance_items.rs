// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_compliance_items_input(input: &crate::input::PutComplianceItemsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_compliance_items_input::ser_put_compliance_items_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_compliance_items_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutComplianceItemsOutput, crate::error::PutComplianceItemsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PutComplianceItemsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::PutComplianceItemsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ComplianceTypeCountLimitExceededException" => crate::error::PutComplianceItemsError::ComplianceTypeCountLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::compliance_type_count_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_compliance_type_count_limit_exceeded_exception::de_compliance_type_count_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutComplianceItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::PutComplianceItemsError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::PutComplianceItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidItemContentException" => crate::error::PutComplianceItemsError::InvalidItemContentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_item_content_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_item_content_exception::de_invalid_item_content_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutComplianceItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceId" => crate::error::PutComplianceItemsError::InvalidResourceId({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_resource_id::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_id::de_invalid_resource_id_json_err(response.body().as_ref(), output).map_err(crate::error::PutComplianceItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceType" => crate::error::PutComplianceItemsError::InvalidResourceType({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_resource_type::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_type::de_invalid_resource_type_json_err(response.body().as_ref(), output).map_err(crate::error::PutComplianceItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ItemSizeLimitExceededException" => crate::error::PutComplianceItemsError::ItemSizeLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::item_size_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_item_size_limit_exceeded_exception::de_item_size_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutComplianceItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TotalSizeLimitExceededException" => crate::error::PutComplianceItemsError::TotalSizeLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::total_size_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_total_size_limit_exceeded_exception::de_total_size_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutComplianceItemsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::PutComplianceItemsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_compliance_items_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutComplianceItemsOutput, crate::error::PutComplianceItemsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_compliance_items_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

