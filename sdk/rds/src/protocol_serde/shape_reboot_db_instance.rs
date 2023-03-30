// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_reboot_db_instance_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RebootDbInstanceOutput, crate::error::RebootDBInstanceError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::RebootDBInstanceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::RebootDBInstanceError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DBInstanceNotFound" => crate::error::RebootDBInstanceError::DbInstanceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_instance_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_instance_not_found_fault::de_db_instance_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::RebootDBInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBInstanceState" => crate::error::RebootDBInstanceError::InvalidDbInstanceStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_instance_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_instance_state_fault::de_invalid_db_instance_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::RebootDBInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::RebootDBInstanceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_reboot_db_instance_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RebootDbInstanceOutput, crate::error::RebootDBInstanceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::reboot_db_instance_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_reboot_db_instance::de_reboot_db_instance(response.body().as_ref(), output).map_err(crate::error::RebootDBInstanceError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_reboot_db_instance(inp: &[u8], mut builder: crate::output::reboot_db_instance_output::Builder) -> Result<crate::output::reboot_db_instance_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("RebootDBInstanceResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected RebootDBInstanceResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("RebootDBInstanceResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected RebootDBInstanceResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("DBInstance") /* DBInstance com.amazonaws.rds.synthetic#RebootDBInstanceOutput$DBInstance */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_db_instance::de_db_instance(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_instance(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected RebootDBInstanceResult tag"))
                    };
    Ok(builder)
}

