// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

//! Data for the [`node`](super) API.
//!
//! This module is just an alternative view of the same data provided by the
//! [`availability`](crate::availability) API. It provides more insight into what data the node
//! actually has at present, as opposed to trying to present a perfect view of an abstract chain,
//! fetching data from other sources as needed. It is also more liberal with provided aggregate
//! counts and statistics which may be inaccurate if data is missing.
//!
//! Due to this relationship with the availability module, this module has its own [data source
//! trait](`NodeDataSource`) but not its own update trait. The node data source is expected to read
//! its data from the same underlying database as the availability API, and as such the data is
//! updated implicitly via the [availability API update
//! trait](crate::availability::UpdateAvailabilityData).

use super::query_data::{BlockId, LeafQueryData, SyncStatus};
use crate::{QueryResult, SignatureKey, VidShare};
use async_trait::async_trait;
use hotshot_types::traits::node_implementation::NodeType;

#[async_trait]
pub trait NodeDataSource<Types: NodeType> {
    async fn block_height(&self) -> QueryResult<usize>;
    async fn get_proposals(
        &self,
        proposer: &SignatureKey<Types>,
        limit: Option<usize>,
    ) -> QueryResult<Vec<LeafQueryData<Types>>>;
    async fn count_proposals(&self, proposer: &SignatureKey<Types>) -> QueryResult<usize>;
    async fn count_transactions(&self) -> QueryResult<usize>;
    async fn payload_size(&self) -> QueryResult<usize>;
    async fn vid_share<ID>(&self, id: ID) -> QueryResult<VidShare>
    where
        ID: Into<BlockId<Types>> + Send + Sync;
    async fn sync_status(&self) -> QueryResult<SyncStatus>;
}
