// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_alfresco_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AlfrescoConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.site_url {
        object.key("SiteUrl").string(var_1.as_str());
    }
    if let Some(var_2) = &input.site_id {
        object.key("SiteId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.secret_arn {
        object.key("SecretArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.ssl_certificate_s3_path {
        #[allow(unused_mut)]
        let mut object_5 = object.key("SslCertificateS3Path").start_object();
        crate::protocol_serde::shape_s3_path::ser_s3_path(&mut object_5, var_4)?;
        object_5.finish();
    }
    if input.crawl_system_folders {
        object.key("CrawlSystemFolders").boolean(input.crawl_system_folders);
    }
    if input.crawl_comments {
        object.key("CrawlComments").boolean(input.crawl_comments);
    }
    if let Some(var_6) = &input.entity_filter {
        let mut array_7 = object.key("EntityFilter").start_array();
        for item_8 in var_6 {
             {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.document_library_field_mappings {
        let mut array_10 = object.key("DocumentLibraryFieldMappings").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.blog_field_mappings {
        let mut array_14 = object.key("BlogFieldMappings").start_array();
        for item_15 in var_13 {
             {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.wiki_field_mappings {
        let mut array_18 = object.key("WikiFieldMappings").start_array();
        for item_19 in var_17 {
             {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.inclusion_patterns {
        let mut array_22 = object.key("InclusionPatterns").start_array();
        for item_23 in var_21 {
             {
                array_22.value().string(item_23.as_str());
            }
        }
        array_22.finish();
    }
    if let Some(var_24) = &input.exclusion_patterns {
        let mut array_25 = object.key("ExclusionPatterns").start_array();
        for item_26 in var_24 {
             {
                array_25.value().string(item_26.as_str());
            }
        }
        array_25.finish();
    }
    if let Some(var_27) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_28 = object.key("VpcConfiguration").start_object();
        crate::protocol_serde::shape_data_source_vpc_configuration::ser_data_source_vpc_configuration(&mut object_28, var_27)?;
        object_28.finish();
    }
    Ok(())
}

pub(crate) fn de_alfresco_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AlfrescoConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::alfresco_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SiteUrl" => {
                                builder = builder.set_site_url(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SiteId" => {
                                builder = builder.set_site_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SecretArn" => {
                                builder = builder.set_secret_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SslCertificateS3Path" => {
                                builder = builder.set_ssl_certificate_s3_path(
                                    crate::protocol_serde::shape_s3_path::de_s3_path(tokens)?
                                );
                            }
                            "CrawlSystemFolders" => {
                                builder = builder.set_crawl_system_folders(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CrawlComments" => {
                                builder = builder.set_crawl_comments(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "EntityFilter" => {
                                builder = builder.set_entity_filter(
                                    crate::protocol_serde::shape_entity_filter::de_entity_filter(tokens)?
                                );
                            }
                            "DocumentLibraryFieldMappings" => {
                                builder = builder.set_document_library_field_mappings(
                                    crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(tokens)?
                                );
                            }
                            "BlogFieldMappings" => {
                                builder = builder.set_blog_field_mappings(
                                    crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(tokens)?
                                );
                            }
                            "WikiFieldMappings" => {
                                builder = builder.set_wiki_field_mappings(
                                    crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(tokens)?
                                );
                            }
                            "InclusionPatterns" => {
                                builder = builder.set_inclusion_patterns(
                                    crate::protocol_serde::shape_data_source_inclusions_exclusions_strings::de_data_source_inclusions_exclusions_strings(tokens)?
                                );
                            }
                            "ExclusionPatterns" => {
                                builder = builder.set_exclusion_patterns(
                                    crate::protocol_serde::shape_data_source_inclusions_exclusions_strings::de_data_source_inclusions_exclusions_strings(tokens)?
                                );
                            }
                            "VpcConfiguration" => {
                                builder = builder.set_vpc_configuration(
                                    crate::protocol_serde::shape_data_source_vpc_configuration::de_data_source_vpc_configuration(tokens)?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

