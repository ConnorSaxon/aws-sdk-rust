// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_get_object_attributes_parts(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::GetObjectAttributesParts, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::GetObjectAttributesParts::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("PartsCount") /* TotalPartsCount com.amazonaws.s3#GetObjectAttributesParts$TotalPartsCount */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3#PartsCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_total_parts_count(var_1);
            }
            ,
            s if s.matches("PartNumberMarker") /* PartNumberMarker com.amazonaws.s3#GetObjectAttributesParts$PartNumberMarker */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_part_number_marker(var_2);
            }
            ,
            s if s.matches("NextPartNumberMarker") /* NextPartNumberMarker com.amazonaws.s3#GetObjectAttributesParts$NextPartNumberMarker */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_part_number_marker(var_3);
            }
            ,
            s if s.matches("MaxParts") /* MaxParts com.amazonaws.s3#GetObjectAttributesParts$MaxParts */ =>  {
                let var_4 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3#MaxParts`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_parts(var_4);
            }
            ,
            s if s.matches("IsTruncated") /* IsTruncated com.amazonaws.s3#GetObjectAttributesParts$IsTruncated */ =>  {
                let var_5 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3#IsTruncated`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_truncated(var_5);
            }
            ,
            s if s.matches("Part") /* Parts com.amazonaws.s3#GetObjectAttributesParts$Parts */ =>  {
                let var_6 =
                    Some(
                        Result::<std::vec::Vec<crate::model::ObjectPart>, aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_7 = builder.parts.take().unwrap_or_default();
                            list_7.push(
                                crate::protocol_serde::shape_object_part::de_object_part(&mut tag)
                                ?
                            );
                            list_7
                        })
                        ?
                    )
                ;
                builder = builder.set_parts(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

