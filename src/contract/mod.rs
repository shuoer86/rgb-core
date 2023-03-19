// RGB Core Library: consensus layer for RGB smart contracts.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2019-2023 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod global;
pub mod data;
pub mod fungible;
pub mod attachment;
mod state;
pub mod seal;
pub mod assignments;
mod operations;
mod bundle;
mod contract;

pub use assignments::{
    Assign, AssignAttach, AssignData, AssignFungible, AssignRights, Assignments, AssignmentsRef,
    TypedAssigns,
};
pub use attachment::AttachId;
pub use bundle::{BundleId, BundledTransition, TransitionBundle};
pub use contract::{
    AttachOutput, ContractHistory, ContractState, DataOutput, FungibleOutput, GlobalOrd, Opout,
    OpoutParseError, OrderedTxid, OutputAssignment, RightsOutput,
};
pub use fungible::{
    BlindingFactor, FieldOrderOverflow, FungibleState, NoiseDumb, PedersenCommitment, RangeProof,
    RangeProofError,
};
pub use global::{GlobalState, GlobalValues};
pub use operations::{
    ContractId, Extension, Genesis, OpId, OpRef, Operation, PrevOuts, Redeemed, Transition,
    Valencies,
};
pub use seal::{ConfidentialSeal, ExposedSeal, GenesisSeal, GraphSeal, SecretSeal, TxoSeal};
pub use state::{ConfidentialState, ExposedState, StateCommitment, StateData, StateType};
