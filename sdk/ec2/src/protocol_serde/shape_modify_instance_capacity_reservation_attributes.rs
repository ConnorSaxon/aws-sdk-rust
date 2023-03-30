// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_instance_capacity_reservation_attributes_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyInstanceCapacityReservationAttributesOutput, crate::error::ModifyInstanceCapacityReservationAttributesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ModifyInstanceCapacityReservationAttributesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::ModifyInstanceCapacityReservationAttributesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_instance_capacity_reservation_attributes_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyInstanceCapacityReservationAttributesOutput, crate::error::ModifyInstanceCapacityReservationAttributesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::modify_instance_capacity_reservation_attributes_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_instance_capacity_reservation_attributes::de_modify_instance_capacity_reservation_attributes(response.body().as_ref(), output).map_err(crate::error::ModifyInstanceCapacityReservationAttributesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_instance_capacity_reservation_attributes(inp: &[u8], mut builder: crate::output::modify_instance_capacity_reservation_attributes_output::Builder) -> Result<crate::output::modify_instance_capacity_reservation_attributes_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ModifyInstanceCapacityReservationAttributesResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ModifyInstanceCapacityReservationAttributesResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("return") /* Return com.amazonaws.ec2.synthetic#ModifyInstanceCapacityReservationAttributesOutput$Return */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_return(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

