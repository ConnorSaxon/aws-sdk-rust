// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_host_reservations_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeHostReservationsOutput, crate::error::DescribeHostReservationsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeHostReservationsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::DescribeHostReservationsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_host_reservations_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeHostReservationsOutput, crate::error::DescribeHostReservationsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_host_reservations_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_host_reservations::de_describe_host_reservations(response.body().as_ref(), output).map_err(crate::error::DescribeHostReservationsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_host_reservations(inp: &[u8], mut builder: crate::output::describe_host_reservations_output::Builder) -> Result<crate::output::describe_host_reservations_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeHostReservationsResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeHostReservationsResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("hostReservationSet") /* HostReservationSet com.amazonaws.ec2.synthetic#DescribeHostReservationsOutput$HostReservationSet */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_host_reservation_set::de_host_reservation_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_host_reservation_set(var_1);
            }
            ,
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#DescribeHostReservationsOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

