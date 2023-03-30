// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_async_response_details(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::AsyncResponseDetails, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::AsyncResponseDetails::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("MultiRegionAccessPointDetails") /* MultiRegionAccessPointDetails com.amazonaws.s3control#AsyncResponseDetails$MultiRegionAccessPointDetails */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_multi_region_access_points_async_response::de_multi_region_access_points_async_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_multi_region_access_point_details(var_1);
            }
            ,
            s if s.matches("ErrorDetails") /* ErrorDetails com.amazonaws.s3control#AsyncResponseDetails$ErrorDetails */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_async_error_details::de_async_error_details(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_error_details(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

