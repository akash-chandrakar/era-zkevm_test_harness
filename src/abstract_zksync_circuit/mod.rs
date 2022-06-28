use sync_vm::circuit_structures::traits::CircuitArithmeticRoundFunction;
use sync_vm::franklin_crypto::bellman::plonk::better_better_cs::gates::selector_optimized_with_d_next::SelectorOptimizedWidth4MainGateWithDNext;
use sync_vm::franklin_crypto::plonk::circuit::custom_rescue_gate::Rescue5CustomGate;
use sync_vm::vm::vm_cycle::entry_point::vm_circuit_entry_point;
use sync_vm::vm::vm_cycle::witness_oracle::WitnessOracle;
use crate::bellman::Engine;
use crate::bellman::plonk::better_better_cs::cs::ConstraintSystem;
use crate::franklin_crypto::plonk::circuit::allocated_num::{AllocatedNum, Num};
use crate::bellman::SynthesisError;
use crossbeam::atomic::AtomicCell;

pub mod concrete_circuits;

use crate::bellman::plonk::better_better_cs::cs::Circuit;
use crate::bellman::plonk::better_better_cs::cs::GateInternal;
use crate::bellman::plonk::better_better_cs::cs::Gate;

// use crate::bellman::bn256::Bn256;

pub trait ZkSyncUniformSynthesisFunction<E: Engine> {
    type Witness: Clone + std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned;
    type Config: Clone + std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned;
    type RoundFunction: CircuitArithmeticRoundFunction<E, 2, 3, StateElement = Num<E>>;

    // fn get_synthesis_function<
    //     CS: ConstraintSystem<E>, 
    //     F: for<'r, 's> FnOnce(&'r mut CS, Option<Self::Witness>, &'s Self::RoundFunction, Self::Config) -> Result<AllocatedNum<E>, SynthesisError>
    // >() -> F;

    fn get_synthesis_function_dyn<
        'a, 
        CS: ConstraintSystem<E> + 'a,
    >() -> Box<dyn FnOnce(&mut CS, Option<Self::Witness>, &Self::RoundFunction, Self::Config) -> Result<AllocatedNum<E>, SynthesisError> + 'a>;
}

pub struct ZkSyncUniformCircuitCircuitInstance<
    E: Engine,
    S: ZkSyncUniformSynthesisFunction<E>,
> {
    pub witness: AtomicCell<Option<S::Witness>>,
    pub config: std::sync::Arc<S::Config>,
    pub round_function: std::sync::Arc<S::RoundFunction>,
}

// fn serialize_atomic_cell<T: serde::Serialize, S>(t: &AtomicCell<Option<T>>, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
//     let el = t.take();
//     let res = match &el {
//         Some(el) => serializer.serialize_some(el),
//         None => serializer.serialize_none(),
//     };
    
//     t.store(el);

//     res
// }

// fn serialize_arc<T: serde::Serialize, S>(t: std::sync::Arc<T>, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
//     (*t).serialize(serializer)
// }

// fn deserialize_atomic_cell<'de, D, T: serde::Deserialize<'de>>(deserializer: D) -> Result<AtomicCell<T>, D::Error> where D: serde::Deserializer<'de> {
//     let res = T::deserialize(deserializer)?;
//     let cell = AtomicCell::new(res);

//     Ok(cell)
// }

// fn deserialize_arc<'de, D, T: serde::Deserialize<'de>>(deserializer: D) -> Result<std::sync::Arc<T>, D::Error> where D: serde::Deserializer<'de> {
//     let res = T::deserialize(deserializer)?;
//     let arc = std::sync::Arc::new(res);

//     Ok(arc)
// }

impl<
    E: Engine, 
    S: ZkSyncUniformSynthesisFunction<E>,
> Clone for ZkSyncUniformCircuitCircuitInstance<E, S> { 
    fn clone(&self) -> Self {
        let wit = self.witness.take();
        let ww = wit.clone();
        self.witness.store(wit);

        Self {
            witness: AtomicCell::new(ww),
            config: std::sync::Arc::clone(&self.config),
            round_function: std::sync::Arc::clone(&self.round_function)
        }
    }
}


impl<
    E: Engine, 
    S: ZkSyncUniformSynthesisFunction<E>,
> Circuit<E> for ZkSyncUniformCircuitCircuitInstance<E, S> {
    type MainGate = SelectorOptimizedWidth4MainGateWithDNext;
    // always two gates
    fn declare_used_gates() -> Result<Vec<Box<dyn GateInternal<E>>>, SynthesisError> {
        Ok(
            vec![
                Self::MainGate::default().into_internal(),
                Rescue5CustomGate::default().into_internal()
            ]
        )
    }
    fn synthesize<CS: ConstraintSystem<E>>(&self, cs: &mut CS) -> Result<(), SynthesisError> {
        let witness = self.witness.take();
        let config: S::Config = (*self.config).clone();
        let round_function = &*self.round_function;
        let synthesis_fn = S::get_synthesis_function_dyn();
        // let synthesis_fn = S::get_synthesis_function();
        let _public_input_var = synthesis_fn(cs, witness, round_function, config)?;

        Ok(())
    }
}