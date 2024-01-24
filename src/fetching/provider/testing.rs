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

#![cfg(any(test, feature = "testing"))]

use super::Provider;
use crate::fetching::Request;
use async_compatibility_layer::async_primitives::broadcast::{channel, BroadcastSender};
use async_std::sync::{Arc, RwLock};
use derivative::Derivative;
use hotshot_types::traits::node_implementation::NodeType;
use std::fmt::Debug;

/// Adaptor to add test-only functionality to an existing [`Provider`].
///
/// [`TestProvider`] wraps an existing provider `P` and adds some additional functionality which can
/// be useful in tests, such as the ability to inject delays into the handling of fetch requests.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = "P: Debug"))]
pub struct TestProvider<P> {
    inner: Arc<P>,
    unblock: Arc<RwLock<Option<BroadcastSender<()>>>>,
}

impl<P> TestProvider<P> {
    pub fn new(inner: P) -> Self {
        Self {
            inner: Arc::new(inner),
            unblock: Default::default(),
        }
    }

    /// Delay fetch requests until [`unblock`](Self::unblock).
    ///
    /// Fetch requests started after this method returns will block without completing until
    /// [`unblock`](Self::unblock) is called. This can be useful for tests to examine the state of a
    /// data source _before_ a fetch request completes, to check that the subsequent fetch actually
    /// has an effect.
    pub async fn block(&self) {
        let mut unblock = self.unblock.write().await;
        if unblock.is_none() {
            *unblock = Some(channel().0);
        }
    }

    /// Allow blocked fetch requests to proceed.
    ///
    /// Fetch requests which are blocked as a result of a preceding call to [`block`](Self::block)
    /// will become unblocked.
    pub async fn unblock(&self) {
        let mut unblock = self.unblock.write().await;
        if let Some(unblock) = unblock.take() {
            unblock.send_async(()).await.ok();
        }
    }
}

impl<Types, P, T> Provider<Types, T> for TestProvider<P>
where
    Types: NodeType,
    T: Request<Types>,
    P: Provider<Types, T> + Sync,
{
    async fn fetch(&self, req: T) -> Option<T::Response> {
        // Block the request if the user has called `block`.
        let handle = {
            match self.unblock.read().await.as_ref() {
                Some(unblock) => Some(unblock.handle_async().await),
                None => None,
            }
        };
        if let Some(mut handle) = handle {
            tracing::info!("request for {req:?} will block until manually unblocked");
            handle.recv_async().await.ok();
            tracing::info!("request for {req:?} unblocked");
        }

        // Do the request.
        self.inner.fetch(req).await
    }
}
