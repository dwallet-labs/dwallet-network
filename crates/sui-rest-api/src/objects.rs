// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::sync::Arc;

use axum::extract::{Path, State};
use sui_types::{
    base_types::{ObjectID, SequenceNumber},
    object::Object,
};

use crate::{node_state_getter::NodeStateGetter, AppError, Bcs};

pub const GET_OBJECT_PATH: &str = "/objects/:object_id";

pub async fn get_object(
    Path(object_id): Path<ObjectID>,
    State(state): State<Arc<dyn NodeStateGetter>>,
) -> Result<Bcs<Object>, AppError> {
    let object = state
        .get_object(&object_id)?
        .ok_or_else(|| anyhow::anyhow!("object not found"))?;

    Ok(Bcs(object))
}

pub const GET_OBJECT_WITH_VERSION_PATH: &str = "/objects/:object_id/version/:version";

pub async fn get_object_with_version(
    Path((object_id, version)): Path<(ObjectID, SequenceNumber)>,
    State(state): State<Arc<dyn NodeStateGetter>>,
) -> Result<Bcs<Object>, AppError> {
    let object = state
        .get_object_by_key(&object_id, version)?
        .ok_or_else(|| anyhow::anyhow!("object not found"))?;

    Ok(Bcs(object))
}
