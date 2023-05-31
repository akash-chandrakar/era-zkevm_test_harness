use boojum::cs::implementations::pow::NoPow;
use derivative::*;
use boojum::gadgets::recursion::recursive_tree_hasher::*;
use boojum::gadgets::recursion::recursive_transcript::*;
use boojum::gadgets::recursion::circuit_pow::*;
use boojum::cs::implementations::transcript::GoldilocksPoisedon2Transcript;
use zkevm_circuits::base_structures::recursion_query::RecursionQuery;
use crate::circuit_definitions::base_layer::TARGET_CIRCUIT_TRACE_LENGTH;
use zkevm_circuits::recursion::leaf_layer::input::*;
use zkevm_circuits::recursion::leaf_layer::*;
use boojum::cs::implementations::transcript::Transcript;
use super::circuit_def::*;

use super::*;

type F = GoldilocksField;
type P = GoldilocksField;
type TR = GoldilocksPoisedon2Transcript;
type R = Poseidon2Goldilocks;
type CTR = CircuitAlgebraicSpongeBasedTranscript<GoldilocksField, 8, 12, 4, R>;
type EXT = GoldilocksExt2;
type H = GoldilocksPoseidon2Sponge<AbsorbtionModeOverwrite>;
type RH = CircuitGoldilocksPoseidon2Sponge;

#[derive(Derivative, serde::Serialize, serde::Deserialize)]
#[derivative(Clone, Debug(bound = ""))]
#[serde(bound = "")]
// #[serde(bound = "RecursionLeafInstanceWitness<F, H, EXT>: serde::Serialize + serde::de::DeserializeOwned,
//     LeafLayerRecursionConfig<F, H::NonCircuitSimulator, EXT>: serde::Serialize + serde::de::DeserializeOwned,
//     TR::TransciptParameters: serde::Serialize + serde::de::DeserializeOwned")]
pub struct LeafLayerRecursiveCircuit<
POW: RecursivePoWRunner<F>,
> {
   pub witness: RecursionLeafInstanceWitness<F, RH, EXT>,
   pub config: LeafLayerRecursionConfig<F, H, EXT>,
   pub transcript_params: <TR as Transcript<F>>::TransciptParameters,
   pub base_layer_circuit_type: BaseLayerCircuitType,
   pub _marker: std::marker::PhantomData<(R, POW)>
}

impl<
POW: RecursivePoWRunner<F>,
> boojum::cs::traits::circuit::CircuitBuilder<F> for LeafLayerRecursiveCircuit<POW> 
where 
    [(); <LogQuery<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <MemoryQuery<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <DecommitQuery<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <UInt256<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <UInt256<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN + 1]:,
    [(); <ExecutionContextRecord<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <TimestampedStorageLogRecord<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <RecursionQuery<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
{
    fn geometry() -> CSGeometry {
        geometry_for_recursion_step()
    }

    fn lookup_parameters() -> LookupParameters {
        lookup_parameters_recursion_step()
    }
    
    fn configure_builder<T: CsBuilderImpl<F, T>, GC: GateConfigurationHolder<F>, TB: StaticToolboxHolder>(
        builder: CsBuilder<T, F, GC, TB>
    ) -> CsBuilder<T, F, impl GateConfigurationHolder<F>, impl StaticToolboxHolder> {
        configure_builder_recursion_step(builder)
    }
}

impl<
POW: RecursivePoWRunner<F>,
> LeafLayerRecursiveCircuit<POW> 
where 
    [(); <LogQuery<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <MemoryQuery<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <DecommitQuery<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <UInt256<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <UInt256<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN + 1]:,
    [(); <ExecutionContextRecord<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <TimestampedStorageLogRecord<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
    [(); <RecursionQuery<F> as CSAllocatableExt<F>>::INTERNAL_STRUCT_LEN]:,
{
    pub fn description(&self) -> String {
        format!("Leaf layer circuit for circuit id {}", self.base_layer_circuit_type as u8)
    }
    pub fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (
            Some(TARGET_CIRCUIT_TRACE_LENGTH),
            Some((1 << 26) + (1 << 25))
        )
    }

    pub fn configure_builder_proxy<T: CsBuilderImpl<F, T>, GC: GateConfigurationHolder<F>, TB: StaticToolboxHolder>(
        &self,
        builder: CsBuilder<T, F, GC, TB>
    ) -> CsBuilder<T, F, impl GateConfigurationHolder<F>, impl StaticToolboxHolder> {
        <Self as boojum::cs::traits::circuit::CircuitBuilder<F>>::configure_builder(builder)
    }

    pub fn add_tables<CS: ConstraintSystem<F>>(&self, _cs: &mut CS) {
    }

    pub fn synthesize_into_cs<
        CS: ConstraintSystem<F> + 'static,
    >(
        self,
        cs: &mut CS,
        round_function: &R,
    ) -> [Num<F>; INPUT_OUTPUT_COMMITMENT_LENGTH] {
        let Self {
            witness, 
            config,
            transcript_params,
            ..
        } = self;

        use crate::circuit_definitions::verifier_builder::dyn_recursive_verifier_builder_for_circuit_type;
        let verifier_builder = dyn_recursive_verifier_builder_for_circuit_type::<F, EXT, CS, R>(self.base_layer_circuit_type as u8);
        leaf_layer_recursion_entry_point::<F, CS, R, RH, EXT, TR, CTR, POW>(
            cs, 
            witness, 
            round_function, 
            config, 
            verifier_builder, 
            transcript_params
        )
    }
}

pub type ZkSyncLeafLayerRecursiveCircuit = LeafLayerRecursiveCircuit<
    // GoldilocksField,
    // GoldilocksExt2,
    // ZkSyncDefaultRoundFunction,
    // CircuitGoldilocksPoseidon2Sponge,
    // GoldilocksPoisedon2Transcript,
    // CircuitAlgebraicSpongeBasedTranscript<GoldilocksField, 8, 12, 4, ZkSyncDefaultRoundFunction>,
    NoPow,
>;

use boojum::cs::traits::circuit::CircuitBuilderProxy;

pub type LeafLayerCircuitBuilder<POW> = CircuitBuilderProxy<F, LeafLayerRecursiveCircuit<POW>>; 
pub type ConcreteLeafLayerCircuitBuilder = LeafLayerCircuitBuilder<
    NoPow,
>;