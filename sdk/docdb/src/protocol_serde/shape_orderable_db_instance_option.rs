// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_orderable_db_instance_option(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::OrderableDbInstanceOption, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::OrderableDbInstanceOption::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Engine") /* Engine com.amazonaws.docdb#OrderableDBInstanceOption$Engine */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine(var_1);
            }
            ,
            s if s.matches("EngineVersion") /* EngineVersion com.amazonaws.docdb#OrderableDBInstanceOption$EngineVersion */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine_version(var_2);
            }
            ,
            s if s.matches("DBInstanceClass") /* DBInstanceClass com.amazonaws.docdb#OrderableDBInstanceOption$DBInstanceClass */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_instance_class(var_3);
            }
            ,
            s if s.matches("LicenseModel") /* LicenseModel com.amazonaws.docdb#OrderableDBInstanceOption$LicenseModel */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_license_model(var_4);
            }
            ,
            s if s.matches("AvailabilityZones") /* AvailabilityZones com.amazonaws.docdb#OrderableDBInstanceOption$AvailabilityZones */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_availability_zone_list::de_availability_zone_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zones(var_5);
            }
            ,
            s if s.matches("Vpc") /* Vpc com.amazonaws.docdb#OrderableDBInstanceOption$Vpc */ =>  {
                let var_6 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.docdb#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_vpc(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

