use super::*;

use crate::GoldilocksField;
use circuit_definitions::aux_definitions::witness_oracle::VmWitnessOracle;
use crate::ZkSyncDefaultRoundFunction;
use boojum::worker::Worker;
use boojum::cs::implementations::polynomial_storage::*;
use boojum::cs::implementations::verifier::*;
use boojum::algebraic_props::sponge::GoldilocksPoseidon2Sponge;
use boojum::cs::oracle::merkle_tree::*;
use boojum::cs::implementations::hints::*;
use boojum::gadgets::recursion::recursive_tree_hasher::CircuitGoldilocksPoseidon2Sponge;
use boojum::config::*;
use boojum::algebraic_props::round_function::AbsorbtionModeOverwrite;
use boojum::field::goldilocks::GoldilocksExt2;
use boojum::gadgets::recursion::recursive_transcript::CircuitAlgebraicSpongeBasedTranscript;
use circuit_definitions::circuit_definitions::recursion_layer::ZkSyncRecursiveLayerCircuit;
use circuit_definitions::circuit_definitions::base_layer::ZkSyncBaseLayerCircuit;

type F = GoldilocksField;
type P = GoldilocksField;
type TR = GoldilocksPoisedon2Transcript;
type R = Poseidon2Goldilocks;
type CTR = CircuitAlgebraicSpongeBasedTranscript<GoldilocksField, 8, 12, 4, R>;
type EXT = GoldilocksExt2;
type H = GoldilocksPoseidon2Sponge<AbsorbtionModeOverwrite>;
type RH = CircuitGoldilocksPoseidon2Sponge;

use boojum::cs::implementations::setup::FinalizationHintsForProver;

pub fn create_base_layer_setup_data(
    circuit: ZkSyncBaseLayerCircuit<GoldilocksField, VmWitnessOracle<GoldilocksField>, ZkSyncDefaultRoundFunction>,
    worker: &Worker,
    fri_lde_factor: usize,
    merkle_tree_cap_size: usize,
) -> (
    SetupBaseStorage<F, P>,
    SetupStorage<F, P>, 
    VerificationKey<F, H>, 
    MerkleTreeWithCap<F, H>,
    DenseVariablesCopyHint,
    DenseWitnessCopyHint,
    FinalizationHintsForProver,
){
    use boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
    use boojum::config::DevCSConfig;
    use boojum::cs::cs_builder::new_builder;
    use boojum::cs::traits::circuit::Circuit;

    let geometry = circuit.geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GoldilocksField, P, SetupCSConfig>::new(
        geometry, 
        num_vars.unwrap(), 
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GoldilocksField>(builder_impl);

    let (mut cs, finalization_hint) = match circuit {
        ZkSyncBaseLayerCircuit::MainVM(inner) => {
            // create_base_layer_setup_data_single(
            //     (*inner.config).clone(),
            //     &worker,
            //     fri_lde_factor,
            //     merkle_tree_cap_size
            // )
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::CodeDecommittmentsSorter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::CodeDecommitter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::LogDemuxer(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::KeccakRoundFunction(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::Sha256RoundFunction(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::ECRecover(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::RAMPermutation(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::StorageSorter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::StorageApplication(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::EventsSorter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncBaseLayerCircuit::L1MessagesSorter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
    };

    let (            
        setup_base,
        setup,
        vk,
        setup_tree,
        vars_hint,
        witness_hints
    ) = cs.get_full_setup(
        worker,
        fri_lde_factor,
        merkle_tree_cap_size,
    );

    (
        setup_base,
        setup,
        vk,
        setup_tree,
        vars_hint,
        witness_hints,
        finalization_hint
    )
}

use boojum::cs::implementations::transcript::GoldilocksPoisedon2Transcript;
use boojum::cs::implementations::prover::ProofConfig;
use boojum::cs::implementations::proof::Proof;

use boojum::cs::implementations::pow::PoWRunner;

pub fn prove_base_layer_circuit<
    POW: PoWRunner
>
(
    circuit: ZkSyncBaseLayerCircuit<GoldilocksField, VmWitnessOracle<GoldilocksField>, ZkSyncDefaultRoundFunction>,
    worker: &Worker,
    proof_config: ProofConfig,
    setup_base: &SetupBaseStorage<F, P>, 
    setup: &SetupStorage<F, P>, 
    setup_tree: &MerkleTreeWithCap<F, H>, 
    vk: &VerificationKey<F, H>,
    vars_hint: &DenseVariablesCopyHint,
    wits_hint: &DenseWitnessCopyHint,
    finalization_hint: &FinalizationHintsForProver,
) -> Proof<F, H, EXT> {
    use boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
    use boojum::cs::cs_builder::new_builder;
    use boojum::cs::traits::circuit::Circuit;

    let geometry = circuit.geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GoldilocksField, P, ProvingCSConfig>::new(
        geometry, 
        num_vars.unwrap(), 
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GoldilocksField>(builder_impl);

    let mut cs = match circuit {
        ZkSyncBaseLayerCircuit::MainVM(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::CodeDecommittmentsSorter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::CodeDecommitter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::LogDemuxer(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::KeccakRoundFunction(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::Sha256RoundFunction(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::ECRecover(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::RAMPermutation(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::StorageSorter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::StorageApplication(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::EventsSorter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncBaseLayerCircuit::L1MessagesSorter(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables_proxy(&mut cs);
            inner.synthesize_proxy(&mut cs);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
    };

    cs.prove_from_precomputations::<EXT, TR, H, POW>(
        proof_config,
        setup_base,
        setup,
        setup_tree,
        vk,
        vars_hint,
        wits_hint,
        (),
        worker,
    )
}

pub fn verify_base_layer_proof<
    POW: PoWRunner
>
(
    circuit: &ZkSyncBaseLayerCircuit<GoldilocksField, VmWitnessOracle<GoldilocksField>, ZkSyncDefaultRoundFunction>,
    proof: &Proof<F, H, EXT>,
    vk: &VerificationKey<F, H>,
) -> bool {
    use circuit_definitions::circuit_definitions::verifier_builder::dyn_verifier_builder_for_circuit_type;

    let verifier_builder = dyn_verifier_builder_for_circuit_type::<F, EXT, ZkSyncDefaultRoundFunction>(circuit.numeric_circuit_type());
    let verifier = verifier_builder.create_verifier();
    // let verifier = verifier_builder.create_dyn_verifier();
    verifier.verify::<H, TR, POW>((), vk, proof)
}

use boojum::gadgets::traits::allocatable::CSAllocatableExt;
use zkevm_circuits::base_structures::log_query::*;
use zkevm_circuits::base_structures::memory_query::*;
use zkevm_circuits::base_structures::decommit_query::*;
use zkevm_circuits::base_structures::recursion_query::*;
use boojum::gadgets::u256::UInt256;
use zkevm_circuits::base_structures::vm_state::saved_context::ExecutionContextRecord;
use zkevm_circuits::storage_validity_by_grand_product::TimestampedStorageLogRecord;

pub fn create_recursive_layer_setup_data(
    circuit: ZkSyncRecursiveLayerCircuit,
    worker: &Worker,
    fri_lde_factor: usize,
    merkle_tree_cap_size: usize,
) -> (
    SetupBaseStorage<F, P>,
    SetupStorage<F, P>, 
    VerificationKey<F, H>, 
    MerkleTreeWithCap<F, H>,
    DenseVariablesCopyHint,
    DenseWitnessCopyHint,
    FinalizationHintsForProver,
) {
    use boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
    use boojum::cs::cs_builder::new_builder;
    use boojum::cs::traits::circuit::Circuit;

    let round_function = ZkSyncDefaultRoundFunction::default();

    let geometry = circuit.geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GoldilocksField, P, SetupCSConfig>::new(
        geometry, 
        num_vars.unwrap(), 
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GoldilocksField>(builder_impl);

    let (mut cs, finalization_hint) = match circuit {
        ZkSyncRecursiveLayerCircuit::SchedulerCircuit(inner) => {
            unreachable!()
            // let builder = inner.configure_builder(builder);
            // let mut cs = builder.build(());
            // inner.add_tables(&mut cs);
            // inner.synthesize_proxy(&mut cs);
            // let (_, finalization_hint) = cs.pad_and_shrink();
            // (cs.into_assembly(), finalization_hint)
        },
        ZkSyncRecursiveLayerCircuit::NodeLayerCircuit(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables(&mut cs);
            inner.synthesize_into_cs(&mut cs, &round_function);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
        ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForMainVM(inner) 
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForCodeDecommittmentsSorter(inner) 
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForCodeDecommitter(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForLogDemuxer(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForKeccakRoundFunction(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForSha256RoundFunction(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForECRecover(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForRAMPermutation(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForStorageSorter(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForStorageApplication(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForEventsSorter(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForL1MessagesSorter(inner)
        => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables(&mut cs);
            inner.synthesize_into_cs(&mut cs, &round_function);
            let (_, finalization_hint) = cs.pad_and_shrink();
            (cs.into_assembly(), finalization_hint)
        },
    };

    let (            
        setup_base,
        setup,
        vk,
        setup_tree,
        vars_hint,
        witness_hints
    ) = cs.get_full_setup(
        worker,
        fri_lde_factor,
        merkle_tree_cap_size,
    );

    (
        setup_base,
        setup,
        vk,
        setup_tree,
        vars_hint,
        witness_hints,
        finalization_hint
    )
}

pub fn prove_recursion_layer_circuit<
    POW: PoWRunner
>(
    circuit: ZkSyncRecursiveLayerCircuit,
    worker: &Worker,
    proof_config: ProofConfig,
    setup_base: &SetupBaseStorage<F, P>, 
    setup: &SetupStorage<F, P>, 
    setup_tree: &MerkleTreeWithCap<F, H>, 
    vk: &VerificationKey<F, H>,
    vars_hint: &DenseVariablesCopyHint,
    wits_hint: &DenseWitnessCopyHint,
    finalization_hint: &FinalizationHintsForProver,
) -> Proof<F, H, EXT> {
    use boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
    use boojum::cs::cs_builder::new_builder;
    use boojum::cs::traits::circuit::Circuit;

    let round_function = ZkSyncDefaultRoundFunction::default();

    let geometry = circuit.geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GoldilocksField, P, ProvingCSConfig>::new(
        geometry, 
        num_vars.unwrap(), 
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GoldilocksField>(builder_impl);

    let cs = match circuit {
        ZkSyncRecursiveLayerCircuit::SchedulerCircuit(inner) => {
            unreachable!()
            // let builder = inner.configure_builder(builder);
            // let mut cs = builder.build(());
            // inner.add_tables(&mut cs);
            // inner.synthesize_proxy(&mut cs);
            // let (_, finalization_hint) = cs.pad_and_shrink();
            // (cs.into_assembly(), finalization_hint)
        },
        ZkSyncRecursiveLayerCircuit::NodeLayerCircuit(inner) => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables(&mut cs);
            inner.synthesize_into_cs(&mut cs, &round_function);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
        ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForMainVM(inner) 
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForCodeDecommittmentsSorter(inner) 
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForCodeDecommitter(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForLogDemuxer(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForKeccakRoundFunction(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForSha256RoundFunction(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForECRecover(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForRAMPermutation(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForStorageSorter(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForStorageApplication(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForEventsSorter(inner)
        | ZkSyncRecursiveLayerCircuit::LeafLayerCircuitForL1MessagesSorter(inner) 
        => {
            let builder = inner.configure_builder_proxy(builder);
            let mut cs = builder.build(());
            inner.add_tables(&mut cs);
            inner.synthesize_into_cs(&mut cs, &round_function);
            cs.pad_and_shrink_using_hint(finalization_hint);
            cs.into_assembly()
        },
    };

    cs.prove_from_precomputations::<EXT, TR, H, POW>(
        proof_config,
        setup_base,
        setup,
        setup_tree,
        vk,
        vars_hint,
        wits_hint,
        (),
        worker,
    )
}

pub fn verify_recursion_layer_proof<
    POW: PoWRunner
>
(
    circuit: &ZkSyncRecursiveLayerCircuit,
    proof: &Proof<F, H, EXT>,
    vk: &VerificationKey<F, H>,
) -> bool {
    let verifier_builder = circuit.into_dyn_verifier_builder();
    let verifier = verifier_builder.create_verifier();
    verifier.verify::<H, TR, POW>((), vk, proof)
}