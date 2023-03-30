// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_status_codes(input: &crate::model::StatusCodes, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.quantity {
        let mut inner_writer = scope.start_el("Quantity").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_1).encode()
        );
    }
    if let Some(var_2) = &input.items {
        let mut inner_writer = scope.start_el("Items").finish();
        for list_item_3 in var_2 {
             {
                let mut inner_writer = inner_writer.start_el("StatusCode").finish();
                inner_writer.data(
                    aws_smithy_types::primitive::Encoder::from(*list_item_3).encode()
                );
            }
        }
    }
    scope.finish();
    Ok(())
}

pub fn de_status_codes(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::StatusCodes, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::StatusCodes::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Quantity") /* Quantity com.amazonaws.cloudfront#StatusCodes$Quantity */ =>  {
                let var_4 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudfront#integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_quantity(var_4);
            }
            ,
            s if s.matches("Items") /* Items com.amazonaws.cloudfront#StatusCodes$Items */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_status_code_list::de_status_code_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_items(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

