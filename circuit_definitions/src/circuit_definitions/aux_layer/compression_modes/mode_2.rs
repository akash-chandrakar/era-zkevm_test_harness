use super::*;

// no lookup, just enough copiable width, moderate LDE factor,
// and matrix multiplication gate + non-linearity gate
pub struct CompressionMode2;

impl ProofCompressionFunction for CompressionMode2 {
    // no PoW from the previous step
    type PreviousLayerPoW = NoPow;

    // no PoW on this step too
    type ThisLayerPoW = NoPow;
    type ThisLayerHasher = H;
    type ThisLayerTranscript = TR;

    fn this_layer_transcript_parameters(
    ) -> <Self::ThisLayerTranscript as Transcript<F>>::TransciptParameters {
        ();
    }

    fn description_for_compression_step() -> String {
        "Compression mode 2: no lookup, just enough copiable width, moderate LDE factor, matrix multiplication gate + non-linearity gate"
        .to_string()
    }

    fn size_hint_for_compression_step() -> (usize, usize) {
        (1 << 14, 1 << 22)
    }

    fn geometry_for_compression_step() -> CSGeometry {
        CSGeometry {
            num_columns_under_copy_permutation: 52,
            // num_witness_columns: 0,
            num_witness_columns: 78,
            num_constant_columns: 4,
            max_allowed_constraint_degree: 8,
        }
    }

    fn lookup_parameters_for_compression_step() -> LookupParameters {
        LookupParameters::NoLookup
    }

    fn configure_builder_for_compression_step<
        T: CsBuilderImpl<F, T>,
        GC: GateConfigurationHolder<F>,
        TB: StaticToolboxHolder,
    >(
        builder: CsBuilder<T, F, GC, TB>,
    ) -> CsBuilder<T, F, impl GateConfigurationHolder<F>, impl StaticToolboxHolder> {
        let builder = ConstantsAllocatorGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = BooleanConstraintGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder =
            R::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);
        // use crate::boojum::gadgets::traits::configuration::ConfigurationFunction;
        // let configuration_function = R::make_specialization_function_0();
        // let builder =
        //     configuration_function.configure_proxy(builder, GatePlacementStrategy::UseGeneralPurposeColumns);
        let builder = ZeroCheckGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
            true,
        );
        let builder = FmaGateInBaseFieldWithoutConstant::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = FmaGateInExtensionWithoutConstant::<F, EXT>::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = SelectionGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = ParallelSelectionGate::<4>::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = PublicInputGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = ReductionGate::<_, 4>::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder =
            NopGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);

        builder
    }

    fn proof_config_for_compression_step() -> ProofConfig {
        ProofConfig {
            fri_lde_factor: 32,
            merkle_tree_cap_size: 64,
            fri_folding_schedule: None,
            security_level: crate::SECURITY_BITS_TARGET,
            pow_bits: 0,
        }
    }

    fn previous_step_builder_for_compression<CS: ConstraintSystem<F> + 'static>(
    ) -> Box<dyn ErasedBuilderForRecursiveVerifier<GoldilocksField, EXT, CS>> {
        use crate::circuit_definitions::aux_layer::compression::CompressionMode1CircuitBuilder;
        CompressionMode1CircuitBuilder::dyn_recursive_verifier_builder::<EXT, CS>()
    }
}