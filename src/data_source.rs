use super::*;
use std::{error::Error, fs::File};

use circuit_definitions::circuit_definitions::base_layer::*;
use circuit_definitions::circuit_definitions::recursion_layer::*;
use circuit_definitions::circuit_definitions::aux_layer::*;

use snark_wrapper::franklin_crypto::bellman::plonk::better_better_cs::proof::Proof as SnarkProof;
use snark_wrapper::franklin_crypto::bellman::plonk::better_better_cs::setup::VerificationKey as SnarkVK;

pub type SourceResult<T> = Result<T, Box<dyn Error>>;

use derivative::*;

// Object save trait to just get things for SYSTEM
pub trait SetupDataSource {
    fn get_base_layer_vk(&self, circuit_type: u8) -> SourceResult<ZkSyncBaseLayerVerificationKey>;
    fn get_base_layer_padding_proof(&self, circuit_type: u8) -> SourceResult<ZkSyncBaseLayerProof>;
    fn get_base_layer_finalization_hint(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncBaseLayerFinalizationHint>;
    fn get_recursion_layer_vk(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncRecursionLayerVerificationKey>;
    fn get_recursion_layer_node_vk(&self) -> SourceResult<ZkSyncRecursionLayerVerificationKey>;
    fn get_recursion_layer_padding_proof(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncRecursionLayerProof>;
    fn get_recursion_layer_finalization_hint(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncRecursionLayerFinalizationHint>;
    fn get_recursion_layer_leaf_padding_proof(&self) -> SourceResult<ZkSyncRecursionLayerProof>;
    fn get_recursion_layer_node_padding_proof(&self) -> SourceResult<ZkSyncRecursionLayerProof>;
    fn get_recursion_layer_node_finalization_hint(
        &self,
    ) -> SourceResult<ZkSyncRecursionLayerFinalizationHint>;
    fn get_compression_vk(&self, circuit_type: u8) -> SourceResult<ZkSyncCompressionLayerVerificationKey>;
    fn get_compression_hint(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncCompressionLayerFinalizationHint>;
    fn get_compression_for_wrapper_vk(&self, circuit_type: u8) -> SourceResult<ZkSyncCompressionForWrapperVerificationKey>;
    fn get_compression_for_wrapper_hint(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncCompressionForWrapperFinalizationHint>;
    fn get_wrapper_vk(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncSnarkWrapperVK>;

    fn set_base_layer_vk(&mut self, vk: ZkSyncBaseLayerVerificationKey) -> SourceResult<()>;
    fn set_base_layer_padding_proof(&mut self, proof: ZkSyncBaseLayerProof) -> SourceResult<()>;
    fn set_base_layer_finalization_hint(
        &mut self,
        hint: ZkSyncBaseLayerFinalizationHint,
    ) -> SourceResult<()>;
    fn set_recursion_layer_vk(
        &mut self,
        vk: ZkSyncRecursionLayerVerificationKey,
    ) -> SourceResult<()>;
    fn set_recursion_layer_node_vk(
        &mut self,
        vk: ZkSyncRecursionLayerVerificationKey,
    ) -> SourceResult<()>;
    fn set_recursion_layer_padding_proof(
        &mut self,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()>;
    fn set_recursion_layer_finalization_hint(
        &mut self,
        hint: ZkSyncRecursionLayerFinalizationHint,
    ) -> SourceResult<()>;
    fn set_recursion_layer_leaf_padding_proof(
        &mut self,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()>;
    fn set_recursion_layer_node_padding_proof(
        &mut self,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()>;
    fn set_recursion_layer_node_finalization_hint(
        &mut self,
        hint: ZkSyncRecursionLayerFinalizationHint,
    ) -> SourceResult<()>;
    fn set_compression_vk(&mut self, vk: ZkSyncCompressionLayerVerificationKey) -> SourceResult<()>;
    fn set_compression_hint(
        &mut self,
        hint: ZkSyncCompressionLayerFinalizationHint,
    ) -> SourceResult<()>;
    fn set_compression_for_wrapper_vk(&mut self, vk: ZkSyncCompressionForWrapperVerificationKey) -> SourceResult<()>;
    fn set_compression_for_wrapper_hint(
        &mut self,
        hint: ZkSyncCompressionForWrapperFinalizationHint,
    ) -> SourceResult<()>;
    fn set_wrapper_vk(
        &mut self,
        vk: ZkSyncSnarkWrapperVK,
    ) -> SourceResult<()>;
}

// Object save trait to just get things for BLOCK
pub trait BlockDataSource {
    fn get_base_layer_proof(
        &self,
        circuit_type: u8,
        index: usize,
    ) -> SourceResult<ZkSyncBaseLayerProof>;
    fn get_leaf_layer_proof(
        &self,
        circuit_type: u8,
        index: usize,
    ) -> SourceResult<ZkSyncRecursionLayerProof>;
    fn get_node_layer_proof(
        &self,
        circuit_type: u8,
        step: usize,
        index: usize,
    ) -> SourceResult<ZkSyncRecursionLayerProof>;
    fn get_scheduler_proof(&self) -> SourceResult<ZkSyncRecursionLayerProof>;
    fn get_compression_proof(&self, circuit_type: u8) -> SourceResult<ZkSyncCompressionLayerProof>;
    fn get_compression_for_wrapper_proof(&self, circuit_type: u8) -> SourceResult<ZkSyncCompressionForWrapperProof>;
    fn get_wrapper_proof(&self, circuit_type: u8) -> SourceResult<ZkSyncSnarkWrapperProof>;

    fn set_base_layer_proof(
        &mut self,
        index: usize,
        proof: ZkSyncBaseLayerProof,
    ) -> SourceResult<()>;
    fn set_leaf_layer_proof(
        &mut self,
        index: usize,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()>;
    fn set_node_layer_proof(
        &mut self,
        circuit_type: u8,
        step: usize,
        index: usize,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()>;
    fn set_scheduler_proof(&mut self, proof: ZkSyncRecursionLayerProof) -> SourceResult<()>;
    fn set_compression_proof(&mut self, proof: ZkSyncCompressionLayerProof) -> SourceResult<()>;
    fn set_compression_for_wrapper_proof(&mut self, proof: ZkSyncCompressionForWrapperProof) -> SourceResult<()>;
    fn set_wrapper_proof(&mut self, proof: ZkSyncSnarkWrapperProof) -> SourceResult<()>;
}

#[derive(Derivative)]
#[derivative(Clone, Copy, Debug)]
pub struct LocalFileDataSource;

impl SetupDataSource for LocalFileDataSource {
    fn get_base_layer_vk(&self, circuit_type: u8) -> SourceResult<ZkSyncBaseLayerVerificationKey> {
        let file = File::open(format!("./setup/base_layer/vk_{}.json", circuit_type))
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_base_layer_padding_proof(&self, circuit_type: u8) -> SourceResult<ZkSyncBaseLayerProof> {
        let file = File::open(format!(
            "./setup/base_layer/padding_proof_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_base_layer_finalization_hint(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncBaseLayerFinalizationHint> {
        let file = File::open(format!(
            "./setup/base_layer/finalization_hint_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_recursion_layer_vk(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncRecursionLayerVerificationKey> {
        let file = File::open(format!("./setup/recursion_layer/vk_{}.json", circuit_type))
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_recursion_layer_node_vk(&self) -> SourceResult<ZkSyncRecursionLayerVerificationKey> {
        let file = File::open("./setup/recursion_layer/vk_node.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_recursion_layer_padding_proof(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncRecursionLayerProof> {
        let file = File::open(format!(
            "./setup/recursion_layer/padding_proof_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_recursion_layer_leaf_padding_proof(&self) -> SourceResult<ZkSyncRecursionLayerProof> {
        let file = File::open("./setup/recursion_layer/padding_proof_leaf.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_recursion_layer_node_padding_proof(&self) -> SourceResult<ZkSyncRecursionLayerProof> {
        let file = File::open("./setup/recursion_layer/padding_proof_node.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_recursion_layer_finalization_hint(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncRecursionLayerFinalizationHint> {
        let file = File::open(format!(
            "./setup/recursion_layer/finalization_hint_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_recursion_layer_node_finalization_hint(
        &self,
    ) -> SourceResult<ZkSyncRecursionLayerFinalizationHint> {
        let file = File::open("./setup/recursion_layer/finalization_hint_node.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_compression_vk(&self, circuit_type: u8) -> SourceResult<ZkSyncCompressionLayerVerificationKey> {
        let file = File::open(format!(
            "./setup/aux_layer/compression_vk_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_compression_hint(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncCompressionLayerFinalizationHint> {
        let file = File::open(format!(
            "./setup/aux_layer/compression_hint_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_compression_for_wrapper_vk(&self, circuit_type: u8) -> SourceResult<ZkSyncCompressionForWrapperVerificationKey> {
        let file = File::open(format!(
            "./setup/aux_layer/compression_for_wrapper_vk_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_compression_for_wrapper_hint(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncCompressionForWrapperFinalizationHint> {
        let file = File::open(format!(
            "./setup/aux_layer/compression_for_wrapper_hint_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_wrapper_vk(
        &self,
        circuit_type: u8,
    ) -> SourceResult<ZkSyncSnarkWrapperVK> {
        let mut file = File::open(format!(
            "./setup/aux_layer/wrapper_vk_{}.key",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;

        let result = ZkSyncSnarkWrapperVK::from_inner(
            circuit_type,
            SnarkVK::read(&mut file).map_err(|el| Box::new(el) as Box<dyn Error>)?
        );

        Ok(result)
    }

    fn set_base_layer_vk(&mut self, vk: ZkSyncBaseLayerVerificationKey) -> SourceResult<()> {
        let circuit_type = vk.numeric_circuit_type();
        let file = File::create(format!("./setup/base_layer/vk_{}.json", circuit_type))
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &vk).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_base_layer_padding_proof(&mut self, proof: ZkSyncBaseLayerProof) -> SourceResult<()> {
        let circuit_type = proof.numeric_circuit_type();
        let file = File::create(format!(
            "./setup/base_layer/padding_proof_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_base_layer_finalization_hint(
        &mut self,
        hint: ZkSyncBaseLayerFinalizationHint,
    ) -> SourceResult<()> {
        let circuit_type = hint.numeric_circuit_type();
        let file = File::create(format!(
            "./setup/base_layer/finalization_hint_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &hint).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_recursion_layer_vk(
        &mut self,
        vk: ZkSyncRecursionLayerVerificationKey,
    ) -> SourceResult<()> {
        let circuit_type = vk.numeric_circuit_type();
        let file = File::create(format!("./setup/recursion_layer/vk_{}.json", circuit_type))
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &vk).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_recursion_layer_node_vk(
        &mut self,
        vk: ZkSyncRecursionLayerVerificationKey,
    ) -> SourceResult<()> {
        let file = File::create("./setup/recursion_layer/vk_node.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &vk).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_recursion_layer_padding_proof(
        &mut self,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()> {
        let circuit_type = proof.numeric_circuit_type();
        let file = File::create(format!(
            "./setup/recursion_layer/padding_proof_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_recursion_layer_leaf_padding_proof(
        &mut self,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()> {
        let file = File::create("./setup/recursion_layer/padding_proof_leaf.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_recursion_layer_node_padding_proof(
        &mut self,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()> {
        let file = File::create("./setup/recursion_layer/padding_proof_node.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_recursion_layer_finalization_hint(
        &mut self,
        hint: ZkSyncRecursionLayerFinalizationHint,
    ) -> SourceResult<()> {
        let circuit_type = hint.numeric_circuit_type();
        let file = File::create(format!(
            "./setup/recursion_layer/finalization_hint_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &hint).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_recursion_layer_node_finalization_hint(
        &mut self,
        hint: ZkSyncRecursionLayerFinalizationHint,
    ) -> SourceResult<()> {
        let file = File::create("./setup/recursion_layer/finalization_hint_node.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &hint).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_compression_vk(&mut self, vk: ZkSyncCompressionLayerVerificationKey) -> SourceResult<()> {
        let circuit_type = vk.numeric_circuit_type();
        let file = File::create(format!("./setup/aux_layer/compression_vk_{}.json", circuit_type))
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &vk).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_compression_hint(
        &mut self,
        hint: ZkSyncCompressionLayerFinalizationHint,
    ) -> SourceResult<()> {
        let circuit_type = hint.numeric_circuit_type();
        let file = File::create(format!(
            "./setup/aux_layer/compression_hint_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &hint).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_compression_for_wrapper_vk(&mut self, vk: ZkSyncCompressionForWrapperVerificationKey) -> SourceResult<()> {
        let circuit_type = vk.numeric_circuit_type();
        let file = File::create(format!("./setup/aux_layer/compression_for_wrapper_vk_{}.json", circuit_type))
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &vk).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_compression_for_wrapper_hint(
        &mut self,
        hint: ZkSyncCompressionForWrapperFinalizationHint,
    ) -> SourceResult<()> {
        let circuit_type = hint.numeric_circuit_type();
        let file = File::create(format!(
            "./setup/aux_layer/compression_for_wrapper_hint_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &hint).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_wrapper_vk(
        &mut self,
        vk: ZkSyncSnarkWrapperVK,
    ) -> SourceResult<()> {
        let circuit_type = vk.numeric_circuit_type();
        let mut file = File::create(format!(
            "./setup/aux_layer/wrapper_vk_{}.key",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;

        vk.into_inner().write(&mut file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
}

impl BlockDataSource for LocalFileDataSource {
    fn get_base_layer_proof(
        &self,
        circuit_type: u8,
        index: usize,
    ) -> SourceResult<ZkSyncBaseLayerProof> {
        let file = File::open(format!(
            "./test_proofs/base_layer/basic_circuit_proof_{}_{}.json",
            circuit_type, index
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_leaf_layer_proof(
        &self,
        circuit_type: u8,
        index: usize,
    ) -> SourceResult<ZkSyncRecursionLayerProof> {
        let file = File::open(format!(
            "./test_proofs/recursion_layer/leaf_layer_proof_{}_{}.json",
            circuit_type, index
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_node_layer_proof(
        &self,
        circuit_type: u8,
        step: usize,
        index: usize,
    ) -> SourceResult<ZkSyncRecursionLayerProof> {
        let file = File::open(format!(
            "./test_proofs/recursion_layer/node_layer_proof_{}_{}_{}.json",
            circuit_type, step, index
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_scheduler_proof(&self) -> SourceResult<ZkSyncRecursionLayerProof> {
        let file = File::open("./test_proofs/recursion_layer/scheduler_proof.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_compression_proof(&self, circuit_type: u8) -> SourceResult<ZkSyncCompressionLayerProof> {
        let file = File::open(format!(
            "./test_proofs/aux_layer/compression_proof_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_compression_for_wrapper_proof(&self, circuit_type: u8) -> SourceResult<ZkSyncCompressionForWrapperProof> {
        let file = File::open(format!(
            "./test_proofs/aux_layer/compression_for_wrapper_proof_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        let result = serde_json::from_reader(file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(result)
    }
    fn get_wrapper_proof(&self, circuit_type: u8) -> SourceResult<ZkSyncSnarkWrapperProof> {
        let mut file = File::open(format!(
            "./test_proofs/aux_layer/wrapper_proof_{}.proof",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;

        let result = ZkSyncSnarkWrapperProof::from_inner(
            circuit_type,
            SnarkProof::read(&mut file).map_err(|el| Box::new(el) as Box<dyn Error>)?
        );

        Ok(result)
    }

    fn set_base_layer_proof(
        &mut self,
        index: usize,
        proof: ZkSyncBaseLayerProof,
    ) -> SourceResult<()> {
        let circuit_type = proof.numeric_circuit_type();
        let file = File::create(format!(
            "./test_proofs/base_layer/basic_circuit_proof_{}_{}.json",
            circuit_type, index
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_leaf_layer_proof(
        &mut self,
        index: usize,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()> {
        let circuit_type = proof.numeric_circuit_type();
        let file = File::create(format!(
            "./test_proofs/recursion_layer/leaf_layer_proof_{}_{}.json",
            circuit_type, index
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_node_layer_proof(
        &mut self,
        circuit_type: u8,
        step: usize,
        index: usize,
        proof: ZkSyncRecursionLayerProof,
    ) -> SourceResult<()> {
        let file = File::create(format!(
            "./test_proofs/recursion_layer/node_layer_proof_{}_{}_{}.json",
            circuit_type, step, index
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_scheduler_proof(&mut self, proof: ZkSyncRecursionLayerProof) -> SourceResult<()> {
        let file = File::create("./test_proofs/recursion_layer/scheduler_proof.json")
            .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_compression_proof(&mut self, proof: ZkSyncCompressionLayerProof) -> SourceResult<()> {
        let circuit_type = proof.numeric_circuit_type();
        let file = File::create(format!(
            "./test_proofs/aux_layer/compression_proof_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_compression_for_wrapper_proof(&mut self, proof: ZkSyncCompressionForWrapperProof) -> SourceResult<()> {
        let circuit_type = proof.numeric_circuit_type();
        let file = File::create(format!(
            "./test_proofs/aux_layer/compression_for_wrapper_proof_{}.json",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;
        serde_json::to_writer(file, &proof).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
    fn set_wrapper_proof(&mut self, proof: ZkSyncSnarkWrapperProof) -> SourceResult<()> {
        let circuit_type = proof.numeric_circuit_type();
        let mut file = File::create(format!(
            "./test_proofs/aux_layer/wrapper_proof_{}.proof",
            circuit_type
        ))
        .map_err(|el| Box::new(el) as Box<dyn Error>)?;

        proof.into_inner().write(&mut file).map_err(|el| Box::new(el) as Box<dyn Error>)?;

        Ok(())
    }
}
