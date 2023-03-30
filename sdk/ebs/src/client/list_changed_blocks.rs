// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListChangedBlocks`](crate::client::fluent_builders::ListChangedBlocks) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListChangedBlocks::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`first_snapshot_id(impl Into<String>)`](crate::client::fluent_builders::ListChangedBlocks::first_snapshot_id) / [`set_first_snapshot_id(Option<String>)`](crate::client::fluent_builders::ListChangedBlocks::set_first_snapshot_id): <p>The ID of the first snapshot to use for the comparison.</p> <important>   <p>The <code>FirstSnapshotID</code> parameter must be specified with a <code>SecondSnapshotId</code> parameter; otherwise, an error occurs.</p>  </important>
    ///   - [`second_snapshot_id(impl Into<String>)`](crate::client::fluent_builders::ListChangedBlocks::second_snapshot_id) / [`set_second_snapshot_id(Option<String>)`](crate::client::fluent_builders::ListChangedBlocks::set_second_snapshot_id): <p>The ID of the second snapshot to use for the comparison.</p> <important>   <p>The <code>SecondSnapshotId</code> parameter must be specified with a <code>FirstSnapshotID</code> parameter; otherwise, an error occurs.</p>  </important>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListChangedBlocks::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListChangedBlocks::set_next_token): <p>The token to request the next page of results.</p>  <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListChangedBlocks::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListChangedBlocks::set_max_results): <p>The maximum number of blocks to be returned by the request.</p>  <p>Even if additional blocks can be retrieved from the snapshot, the request can return less blocks than <b>MaxResults</b> or an empty array of blocks.</p>  <p>To retrieve the next set of blocks from the snapshot, make another request with the returned <b>NextToken</b> value. The value of <b>NextToken</b> is <code>null</code> when there are no more blocks to return.</p>
    ///   - [`starting_block_index(i32)`](crate::client::fluent_builders::ListChangedBlocks::starting_block_index) / [`set_starting_block_index(Option<i32>)`](crate::client::fluent_builders::ListChangedBlocks::set_starting_block_index): <p>The block index from which the comparison should start.</p>  <p>The list in the response will start from this block index or the next valid block index in the snapshots.</p>  <p>If you specify <b>NextToken</b>, then <b>StartingBlockIndex</b> is ignored.</p>
                            /// - On success, responds with [`ListChangedBlocksOutput`](crate::output::ListChangedBlocksOutput) with field(s):
    ///   - [`changed_blocks(Option<Vec<ChangedBlock>>)`](crate::output::ListChangedBlocksOutput::changed_blocks): <p>An array of objects containing information about the changed blocks.</p>
    ///   - [`expiry_time(Option<DateTime>)`](crate::output::ListChangedBlocksOutput::expiry_time): <p>The time when the <code>BlockToken</code> expires.</p>
    ///   - [`volume_size(Option<i64>)`](crate::output::ListChangedBlocksOutput::volume_size): <p>The size of the volume in GB.</p>
    ///   - [`block_size(Option<i32>)`](crate::output::ListChangedBlocksOutput::block_size): <p>The size of the blocks in the snapshot, in bytes.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListChangedBlocksOutput::next_token): <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<ListChangedBlocksError>`](crate::error::ListChangedBlocksError)
    pub fn list_changed_blocks(&self) -> crate::client::fluent_builders::ListChangedBlocks {
                                crate::client::fluent_builders::ListChangedBlocks::new(self.handle.clone())
                            }
}

