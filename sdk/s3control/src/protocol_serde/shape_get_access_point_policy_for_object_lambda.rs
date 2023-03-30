// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_access_point_policy_for_object_lambda_headers(
                    input: &crate::input::GetAccessPointPolicyForObjectLambdaInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.account_id {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("account_id", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-account-id", header_value);
                        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_access_point_policy_for_object_lambda_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetAccessPointPolicyForObjectLambdaOutput, crate::error::GetAccessPointPolicyForObjectLambdaError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetAccessPointPolicyForObjectLambdaError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::GetAccessPointPolicyForObjectLambdaError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_access_point_policy_for_object_lambda_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetAccessPointPolicyForObjectLambdaOutput, crate::error::GetAccessPointPolicyForObjectLambdaError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_access_point_policy_for_object_lambda_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_access_point_policy_for_object_lambda::de_get_access_point_policy_for_object_lambda(response.body().as_ref(), output).map_err(crate::error::GetAccessPointPolicyForObjectLambdaError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_access_point_policy_for_object_lambda(inp: &[u8], mut builder: crate::output::get_access_point_policy_for_object_lambda_output::Builder) -> Result<crate::output::get_access_point_policy_for_object_lambda_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("GetAccessPointPolicyForObjectLambdaResult") {
                            return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetAccessPointPolicyForObjectLambdaResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Policy") /* Policy com.amazonaws.s3control.synthetic#GetAccessPointPolicyForObjectLambdaOutput$Policy */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_policy(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

