// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CompleteMultipartUpload`](crate::client::fluent_builders::CompleteMultipartUpload) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_bucket): <p>Name of the bucket to which the multipart upload was initiated.</p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`key(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::key) / [`set_key(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_key): <p>Object key for which the multipart upload was initiated.</p>
    ///   - [`multipart_upload(CompletedMultipartUpload)`](crate::client::fluent_builders::CompleteMultipartUpload::multipart_upload) / [`set_multipart_upload(Option<CompletedMultipartUpload>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_multipart_upload): <p>The container for the multipart upload request information.</p>
    ///   - [`upload_id(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::upload_id) / [`set_upload_id(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_upload_id): <p>ID for the initiated multipart upload.</p>
    ///   - [`checksum_crc32(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::checksum_crc32) / [`set_checksum_crc32(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_checksum_crc32): <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_crc32_c(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::checksum_crc32_c) / [`set_checksum_crc32_c(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_checksum_crc32_c): <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha1(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::checksum_sha1) / [`set_checksum_sha1(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_checksum_sha1): <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha256(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::checksum_sha256) / [`set_checksum_sha256(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_checksum_sha256): <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`request_payer(RequestPayer)`](crate::client::fluent_builders::CompleteMultipartUpload::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    ///   - [`sse_customer_algorithm(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::sse_customer_algorithm) / [`set_sse_customer_algorithm(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_sse_customer_algorithm): <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`sse_customer_key(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::sse_customer_key) / [`set_sse_customer_key(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_sse_customer_key): <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`sse_customer_key_md5(impl Into<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::sse_customer_key_md5) / [`set_sse_customer_key_md5(Option<String>)`](crate::client::fluent_builders::CompleteMultipartUpload::set_sse_customer_key_md5): <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
                            /// - On success, responds with [`CompleteMultipartUploadOutput`](crate::output::CompleteMultipartUploadOutput) with field(s):
    ///   - [`location(Option<String>)`](crate::output::CompleteMultipartUploadOutput::location): <p>The URI that identifies the newly created object.</p>
    ///   - [`bucket(Option<String>)`](crate::output::CompleteMultipartUploadOutput::bucket): <p>The name of the bucket that contains the newly created object. Does not return the access point ARN or access point alias if used.</p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`key(Option<String>)`](crate::output::CompleteMultipartUploadOutput::key): <p>The object key of the newly created object.</p>
    ///   - [`expiration(Option<String>)`](crate::output::CompleteMultipartUploadOutput::expiration): <p>If the object expiration is configured, this will contain the expiration date (<code>expiry-date</code>) and rule ID (<code>rule-id</code>). The value of <code>rule-id</code> is URL-encoded.</p>
    ///   - [`e_tag(Option<String>)`](crate::output::CompleteMultipartUploadOutput::e_tag): <p>Entity tag that identifies the newly created object's data. Objects with different object data will have different entity tags. The entity tag is an opaque string. The entity tag may or may not be an MD5 digest of the object data. If the entity tag is not an MD5 digest of the object data, it will contain one or more nonhexadecimal characters and/or will consist of less than 32 or more than 32 hexadecimal digits. For more information about how the entity tag is calculated, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_crc32(Option<String>)`](crate::output::CompleteMultipartUploadOutput::checksum_crc32): <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_crc32_c(Option<String>)`](crate::output::CompleteMultipartUploadOutput::checksum_crc32_c): <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha1(Option<String>)`](crate::output::CompleteMultipartUploadOutput::checksum_sha1): <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha256(Option<String>)`](crate::output::CompleteMultipartUploadOutput::checksum_sha256): <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`server_side_encryption(Option<ServerSideEncryption>)`](crate::output::CompleteMultipartUploadOutput::server_side_encryption): <p>If you specified server-side encryption either with an Amazon S3-managed encryption key or an Amazon Web Services KMS key in your initiate multipart upload request, the response includes this header. It confirms the encryption algorithm that Amazon S3 used to encrypt the object.</p>
    ///   - [`version_id(Option<String>)`](crate::output::CompleteMultipartUploadOutput::version_id): <p>Version ID of the newly created object, in case the bucket has versioning turned on.</p>
    ///   - [`ssekms_key_id(Option<String>)`](crate::output::CompleteMultipartUploadOutput::ssekms_key_id): <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    ///   - [`bucket_key_enabled(bool)`](crate::output::CompleteMultipartUploadOutput::bucket_key_enabled): <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    ///   - [`request_charged(Option<RequestCharged>)`](crate::output::CompleteMultipartUploadOutput::request_charged): <p>If present, indicates that the requester was successfully charged for the request.</p>
                            /// - On failure, responds with [`SdkError<CompleteMultipartUploadError>`](crate::error::CompleteMultipartUploadError)
    pub fn complete_multipart_upload(&self) -> crate::client::fluent_builders::CompleteMultipartUpload {
                                crate::client::fluent_builders::CompleteMultipartUpload::new(self.handle.clone())
                            }
}

