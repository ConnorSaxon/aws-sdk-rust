// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_change_cidr_collection_input_input(input: &crate::input::ChangeCidrCollectionInput, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.changes {
        let mut inner_writer = scope.start_el("Changes").finish();
        for list_item_2 in var_1 {
             {
                let inner_writer = inner_writer.start_el("member");
                crate::protocol_serde::shape_cidr_collection_change::ser_cidr_collection_change(list_item_2, inner_writer)?
            }
        }
    }
    if let Some(var_3) = &input.collection_version {
        let mut inner_writer = scope.start_el("CollectionVersion").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_3).encode()
        );
    }
    scope.finish();
    Ok(())
}

