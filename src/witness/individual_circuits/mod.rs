use super::*;
use super::oracle::*;
use crate::witness::utils::*;

use boojum::field::SmallField;
use boojum::algebraic_props::round_function::AlgebraicRoundFunction;
use boojum::gadgets::traits::round_function::*;
use crate::witness::full_block_artifact::FullBlockArtifacts;
use zkevm_circuits::base_structures::precompile_input_outputs::*;
use boojum::gadgets::queue::CircuitQueueRawWitness;
use std::collections::VecDeque;
use zkevm_circuits::fsm_input_output::*;
use boojum::gadgets::traits::allocatable::CSAllocatable;
use zkevm_circuits::base_structures::vm_state::{QUEUE_STATE_WIDTH, FULL_SPONGE_QUEUE_STATE_WIDTH};
use zkevm_circuits::DEFAULT_NUM_PERMUTATION_ARGUMENT_REPETITIONS;

pub mod ram_permutation;
pub mod decommit_code;
pub mod log_demux;
pub mod storage_sort_dedup;
pub mod events_sort_dedup;
pub mod data_hasher_and_merklizer;
pub mod storage_application;
pub mod keccak256_round_function;
pub mod sha256_round_function;
pub mod ecrecover;
pub mod sort_decommit_requests;

// use std::collections::VecDeque;