#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    use ink_prelude::vec::Vec;
    use ink_prelude::vec;
    use winterfell::{
      math::{fields::f128::BaseElement, FieldElement},
      Air, AirContext, Assertion, ByteWriter, EvaluationFrame, ProofOptions, Serializable,
      TraceInfo, TransitionConstraintDegree, StarkProof
    };
    
    pub struct PublicInputs {
      start: BaseElement,
      result: BaseElement,
    }
    
    impl Serializable for PublicInputs {
      fn write_into<W: ByteWriter>(&self, target: &mut W) {
          target.write(self.start);
          target.write(self.result);
      }
    }
    
    pub struct WorkAir {
      context: AirContext<BaseElement>,
      start: BaseElement,
      result: BaseElement,
    }
    
    impl Air for WorkAir {
      type BaseField = BaseElement;
      type PublicInputs = PublicInputs;
    
      fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: ProofOptions) -> Self {
          assert_eq!(1, trace_info.width());
          let degrees = vec![TransitionConstraintDegree::new(3)];
          let num_assertions = 2;
    
          WorkAir {
              context: AirContext::new(trace_info, degrees, num_assertions, options),
              start: pub_inputs.start,
              result: pub_inputs.result,
          }
      }
    
      fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
          &self,
          frame: &EvaluationFrame<E>,
          _periodic_values: &[E],
          result: &mut [E],
      ) {
          let current_state = &frame.current()[0];
          let next_state = current_state.exp(3u32.into()) + E::from(42u32);
          result[0] = frame.next()[0] - next_state;
      }
    
      fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
          let last_step = self.trace_length() - 1;
          vec![
              Assertion::single(0, 0, self.start),
              Assertion::single(0, last_step, self.result),
          ]
      }
    
      fn context(&self) -> &AirContext<Self::BaseField> {
          &self.context
      }
    }

    #[ink(storage)]
    pub struct Incrementer {}

    impl Incrementer {
        #[ink(constructor)]
        pub fn default() -> Self {
          Self {}
        }

        #[ink(message)]
        pub fn verify(&self, raw_start: u8, raw_result: u8, raw_proof: Vec<u8>) {
          let proof = match StarkProof::from_bytes(&raw_proof) {
            Ok(p) => p,
            Err(_) => return,
          };
          let start = BaseElement::from(raw_start);
          let result = BaseElement::from(raw_result);
          let pub_inputs = PublicInputs { start, result };
          winterfell::verify::<WorkAir>(proof, pub_inputs);
        }
    }
}
