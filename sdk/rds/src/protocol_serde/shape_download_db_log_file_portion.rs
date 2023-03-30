// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_download_db_log_file_portion_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DownloadDbLogFilePortionOutput, crate::error::DownloadDBLogFilePortionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DownloadDBLogFilePortionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DownloadDBLogFilePortionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DBInstanceNotFound" => crate::error::DownloadDBLogFilePortionError::DbInstanceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_instance_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_instance_not_found_fault::de_db_instance_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DownloadDBLogFilePortionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBLogFileNotFoundFault" => crate::error::DownloadDBLogFilePortionError::DbLogFileNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_log_file_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_log_file_not_found_fault::de_db_log_file_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DownloadDBLogFilePortionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DownloadDBLogFilePortionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_download_db_log_file_portion_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DownloadDbLogFilePortionOutput, crate::error::DownloadDBLogFilePortionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::download_db_log_file_portion_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_download_db_log_file_portion::de_download_db_log_file_portion(response.body().as_ref(), output).map_err(crate::error::DownloadDBLogFilePortionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_download_db_log_file_portion(inp: &[u8], mut builder: crate::output::download_db_log_file_portion_output::Builder) -> Result<crate::output::download_db_log_file_portion_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DownloadDBLogFilePortionResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DownloadDBLogFilePortionResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DownloadDBLogFilePortionResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DownloadDBLogFilePortionResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("LogFileData") /* LogFileData com.amazonaws.rds.synthetic#DownloadDBLogFilePortionOutput$LogFileData */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_log_file_data(var_1);
            }
            ,
            s if s.matches("Marker") /* Marker com.amazonaws.rds.synthetic#DownloadDBLogFilePortionOutput$Marker */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_marker(var_2);
            }
            ,
            s if s.matches("AdditionalDataPending") /* AdditionalDataPending com.amazonaws.rds.synthetic#DownloadDBLogFilePortionOutput$AdditionalDataPending */ =>  {
                let var_3 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_additional_data_pending(var_3);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DownloadDBLogFilePortionResult tag"))
                    };
    Ok(builder)
}

