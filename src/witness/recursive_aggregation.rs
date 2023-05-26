use boojum::{field::{goldilocks::GoldilocksExt2, U64Representable}, gadgets::{traits::allocatable::CSAllocatable, recursion::allocated_vk::AllocatedVerificationKey}, cs::traits::cs::ConstraintSystem};

use crate::witness::utils::take_sponge_like_queue_state_from_simulator;
use super::*;
use boojum::gadgets::recursion::recursive_tree_hasher::CircuitGoldilocksPoseidon2Sponge;
use zkevm_circuits::recursion::{leaf_layer::input::*, VK_COMMITMENT_LENGTH};
use boojum::gadgets::queue::full_state_queue::FullStateCircuitQueueRawWitness;
use std::collections::VecDeque;
use boojum::gadgets::num::Num;

type F = GoldilocksField;
type EXT = GoldilocksExt2;
type H = CircuitGoldilocksPoseidon2Sponge;

use boojum::algebraic_props::round_function::AlgebraicRoundFunction;
use boojum::gadgets::traits::round_function::BuildableCircuitRoundFunction;
use zkevm_circuits::fsm_input_output::commit_variable_length_encodable_item;
use boojum::gadgets::traits::encodable::CircuitVarLengthEncodable;
use boojum::gadgets::traits::witnessable::WitnessHookable;
use zkevm_circuits::scheduler::aux::BaseLayerCircuitType;
use zkevm_circuits::recursion::leaf_layer::LeafLayerRecursionConfig;
use crate::external_calls::base_layer_proof_config;
use boojum::gadgets::recursion::recursive_tree_hasher::RecursiveTreeHasher;
use circuit_definitions::circuit_definitions::recursion_layer::*;
use circuit_definitions::circuit_definitions::base_layer::*;
use circuit_definitions::encodings::recursion_request::RecursionQueueSimulator;
use circuit_definitions::circuit_definitions::recursion_layer::leaf_layer::*;
use circuit_definitions::encodings::CircuitEquivalentReflection;

pub(crate) fn compute_encodable_item_from_witness<
T: CSAllocatable<F> + CircuitVarLengthEncodable<F>, 
const N: usize, 
CS: ConstraintSystem<F>,
R: BuildableCircuitRoundFunction<F, 8, 12, 4> + AlgebraicRoundFunction<F, 8, 12, 4> + serde::Serialize + serde::de::DeserializeOwned,
>(
    wit: T::Witness,
    cs: &mut CS,
    round_function: &R,
) -> [F; N] {
    // allocate in full
    
    let element = T::allocate(cs, wit);

    let commitment = commit_variable_length_encodable_item(cs, &element, round_function);
    let commitment = commitment.witness_hook(&*cs)().unwrap();

    commitment
}

pub fn create_leaf_witnesses(
    subset: (u64, RecursionQueueSimulator<F>, Vec<ZkSyncBaseLayerClosedFormInput<F>>),
    proofs: Vec<ZkSyncBaseLayerProof>,
    padding_proof: ZkSyncBaseLayerProof,
    vk: ZkSyncBaseLayerVerificationKey,
) -> (Vec<ZkSyncRecursiveLayerCircuit>, Vec<ZkSyncBaseLayerClosedFormInput<F>>) {
    let round_function = ZkSyncDefaultRoundFunction::default();
    use crate::witness::utils::create_cs_for_witness_generation;
    use crate::witness::utils::*;
    let mut cs_for_witness_generation = create_cs_for_witness_generation::<F, ZkSyncDefaultRoundFunction>(
        TRACE_LEN_LOG_2_FOR_CALCULATION,
        MAX_VARS_LOG_2_FOR_CALCULATION,
    );

    let mut results = vec![];
    let (circuit_type, queue, closed_form_inputs) = subset;
    assert_eq!(queue.num_items as usize, proofs.len());
    assert_eq!(circuit_type, padding_proof.numeric_circuit_type() as u64);
    assert_eq!(circuit_type, vk.numeric_circuit_type() as u64);

    let queue_splits = queue.split_by(RECURSION_ARITY, &round_function);
    let mut proofs_iter = proofs.into_iter();

    let vk_commitment: [_; VK_COMMITMENT_LENGTH] = compute_encodable_item_from_witness::<
        AllocatedVerificationKey<F, H>, 
        VK_COMMITMENT_LENGTH,
        _,
        _,
    >(
        vk.clone().into_inner(),
        &mut cs_for_witness_generation,
        &round_function,
    );

    let params = RecursionLeafParametersWitness::<F> {
        circuit_type: F::from_u64_unchecked(circuit_type),
        vk_commitment: vk_commitment,
    };

    for el in queue_splits.into_iter() {
        let mut proof_witnesses = VecDeque::new();
        for _ in 0..el.num_items {
            let t = proofs_iter.next().expect("proof");
            proof_witnesses.push_back(t.into_inner());
        }
        let leaf_input = RecursionLeafInputWitness::<F> {
            params: params.clone(),
            queue_state: take_sponge_like_queue_state_from_simulator(&el),
        };

        let elements: VecDeque<_> = el.witness.iter().map(|(_, old_tail, element)| {
            (element.reflect(), *old_tail)
        }).collect();

        let witness = RecursionLeafInstanceWitness::<F, H, EXT> {
            input: leaf_input,
            vk_witness: vk.clone().into_inner(),
            queue_witness: FullStateCircuitQueueRawWitness { elements: elements },
            proof_witnesses: proof_witnesses,
        };

        let config = LeafLayerRecursionConfig::<F, <H as RecursiveTreeHasher<F, Num<F>>>::NonCircuitSimulator, EXT> {
            proof_config: base_layer_proof_config(),
            vk_fixed_parameters: vk.clone().into_inner().fixed_parameters,
            capacity: RECURSION_ARITY,
            padding_proof: padding_proof.clone().into_inner(),
        };

        let circuit = match vk.numeric_circuit_type() {
            i if i == BaseLayerCircuitType::VM as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::VM,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForMainVM(circuit)
            },
            i if i == BaseLayerCircuitType::DecommitmentsFilter as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::DecommitmentsFilter,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForCodeDecommittmentsSorter(circuit)
            },
            i if i == BaseLayerCircuitType::Decommiter as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::Decommiter,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForCodeDecommitter(circuit)
            },
            i if i == BaseLayerCircuitType::LogDemultiplexer as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::LogDemultiplexer,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForLogDemuxer(circuit)
            },
            i if i == BaseLayerCircuitType::KeccakPrecompile as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::KeccakPrecompile,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForKeccakRoundFunction(circuit)
            },
            i if i == BaseLayerCircuitType::Sha256Precompile as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::Sha256Precompile,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForSha256RoundFunction(circuit)
            },
            i if i == BaseLayerCircuitType::EcrecoverPrecompile as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::EcrecoverPrecompile,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForECRecover(circuit)
            },
            i if i == BaseLayerCircuitType::RamValidation as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::RamValidation,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForRAMPermutation(circuit)
            },
            i if i == BaseLayerCircuitType::StorageFilter as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::StorageFilter,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForStorageSorter(circuit)
            },
            i if i == BaseLayerCircuitType::StorageApplicator as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::StorageApplicator,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForStorageApplication(circuit)
            },
            i if i == BaseLayerCircuitType::EventsRevertsFilter as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::EventsRevertsFilter,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForEventsSorter(circuit)
            },
            i if i == BaseLayerCircuitType::L1MessagesRevertsFilter as u8 => {
                let circuit = ZkSyncLeafLayerRecursiveCircuit {
                    witness,
                    config,
                    transcript_params: (),
                    base_layer_circuit_type: BaseLayerCircuitType::L1MessagesRevertsFilter,
                    _marker: std::marker::PhantomData,
                };
                ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForL1MessagesSorter(circuit)
            },
            // i if i == BaseLayerCircuitType::VM as u8 => {
            //     ZkSyncUniformCircuitVerifierBuilder::<F, VMMainCircuitVerifierBuilder<F, VmWitnessOracle<F>, R>>::default().into_dyn_verifier_builder()
            // },
            // i if i == BaseLayerCircuitType::VM as u8 => {
            //     ZkSyncUniformCircuitVerifierBuilder::<F, VMMainCircuitVerifierBuilder<F, VmWitnessOracle<F>, R>>::default().into_dyn_verifier_builder()
            // },
            _ => {
                panic!("unknown circuit type = {}", circuit_type);
            }
        };

        results.push(circuit);
    }

    (results, closed_form_inputs)
}

