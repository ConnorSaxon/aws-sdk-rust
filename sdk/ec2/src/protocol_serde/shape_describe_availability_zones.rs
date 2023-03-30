// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_availability_zones_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeAvailabilityZonesOutput, crate::error::DescribeAvailabilityZonesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeAvailabilityZonesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::DescribeAvailabilityZonesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_availability_zones_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeAvailabilityZonesOutput, crate::error::DescribeAvailabilityZonesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_availability_zones_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_availability_zones::de_describe_availability_zones(response.body().as_ref(), output).map_err(crate::error::DescribeAvailabilityZonesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_availability_zones(inp: &[u8], mut builder: crate::output::describe_availability_zones_output::Builder) -> Result<crate::output::describe_availability_zones_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeAvailabilityZonesResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeAvailabilityZonesResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("availabilityZoneInfo") /* AvailabilityZones com.amazonaws.ec2.synthetic#DescribeAvailabilityZonesOutput$AvailabilityZones */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_availability_zone_list::de_availability_zone_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zones(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

