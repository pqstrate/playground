use winterfell::{
    Air, AirContext, Assertion, EvaluationFrame, ProofOptions, Prover,
    TraceInfo, TraceTable, TransitionConstraintDegree, FieldExtension, BatchingMethod,
    math::{fields::f128::BaseElement, FieldElement},
    crypto::{DefaultRandomCoin, ElementHasher, MerkleTree},
    matrix::ColMatrix, AuxRandElements, CompositionPoly, CompositionPolyTrace,
    ConstraintCompositionCoefficients, DefaultConstraintCommitment, DefaultConstraintEvaluator,
    DefaultTraceLde, PartitionOptions, StarkDomain, TracePolyTable,
};
use std::marker::PhantomData;

const TRACE_WIDTH: usize = 2;

pub struct FibLikeAir {
    context: AirContext<BaseElement>,
    result: BaseElement,
}

impl Air for FibLikeAir {
    type BaseField = BaseElement;
    type PublicInputs = BaseElement;

    fn new(trace_info: TraceInfo, pub_inputs: Self::BaseField, options: ProofOptions) -> Self {
        let degrees = vec![TransitionConstraintDegree::new(8), TransitionConstraintDegree::new(1)];
        assert_eq!(TRACE_WIDTH, trace_info.width());
        FibLikeAir {
            context: AirContext::new(trace_info, degrees, 3, options),
            result: pub_inputs,
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();

        debug_assert_eq!(TRACE_WIDTH, current.len());
        debug_assert_eq!(TRACE_WIDTH, next.len());

        // Constraint: next[0] = current[0]^8 + current[1] 
        // Constraint: next[1] = current[0] (shift register)
        let x1_pow8 = current[0] * current[0] * current[0] * current[0] * 
                     current[0] * current[0] * current[0] * current[0];
        
        result[0] = next[0] - (x1_pow8 + current[1]);
        result[1] = next[1] - current[0];
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(0, 0, BaseElement::new(1)), // x1 starts at 1
            Assertion::single(1, 0, BaseElement::new(1)), // x2 starts at 1  
            Assertion::single(0, last_step, self.result), // final result
        ]
    }
}

pub struct FibLikeProver<H: ElementHasher> {
    options: ProofOptions,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> FibLikeProver<H> {
    pub fn new(options: ProofOptions) -> Self {
        Self {
            options,
            _hasher: PhantomData,
        }
    }

    pub fn build_trace(&self, num_steps: usize) -> TraceTable<BaseElement> {
        assert!(num_steps.is_power_of_two());
        
        let mut x1_col = Vec::with_capacity(num_steps);
        let mut x2_col = Vec::with_capacity(num_steps);
        
        let mut x1 = BaseElement::new(1);
        let mut x2 = BaseElement::new(1);
        
        x1_col.push(x1);
        x2_col.push(x2);
        
        for _ in 1..num_steps {
            let next_x1 = x1.exp(8u64.into()) + x2;
            let next_x2 = x1;
            
            x1_col.push(next_x1);
            x2_col.push(next_x2);
            
            x1 = next_x1;
            x2 = next_x2;
        }
        
        TraceTable::init(vec![x1_col, x2_col])
    }
}

impl<H: ElementHasher> Prover for FibLikeProver<H>
where
    H: ElementHasher<BaseField = BaseElement> + Sync,
{
    type BaseField = BaseElement;
    type Air = FibLikeAir;
    type Trace = TraceTable<BaseElement>;
    type HashFn = H;
    type VC = MerkleTree<H>;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;
    type TraceLde<E: FieldElement<BaseField = Self::BaseField>> =
        DefaultTraceLde<E, Self::HashFn, Self::VC>;
    type ConstraintCommitment<E: FieldElement<BaseField = Self::BaseField>> =
        DefaultConstraintCommitment<E, H, Self::VC>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = Self::BaseField>> =
        DefaultConstraintEvaluator<'a, Self::Air, E>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> BaseElement {
        use winterfell::Trace;
        let last_step = trace.length() - 1;
        trace.get(0, last_step)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }

    fn new_trace_lde<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        trace_info: &TraceInfo,
        main_trace: &ColMatrix<Self::BaseField>,
        domain: &StarkDomain<Self::BaseField>,
        partition_option: PartitionOptions,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        DefaultTraceLde::new(trace_info, main_trace, domain, partition_option)
    }

    fn new_evaluator<'a, E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        air: &'a Self::Air,
        aux_rand_elements: Option<AuxRandElements<E>>,
        composition_coefficients: ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        DefaultConstraintEvaluator::new(air, aux_rand_elements, composition_coefficients)
    }

    fn build_constraint_commitment<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        composition_poly_trace: CompositionPolyTrace<E>,
        num_constraint_composition_columns: usize,
        domain: &StarkDomain<Self::BaseField>,
        partition_options: PartitionOptions,
    ) -> (Self::ConstraintCommitment<E>, CompositionPoly<E>) {
        DefaultConstraintCommitment::new(
            composition_poly_trace,
            num_constraint_composition_columns,
            domain,
            partition_options,
        )
    }
}

pub fn run_example(num_steps: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating proof for Fibonacci-like sequence (x1^8 + x2) with {} steps", num_steps);
    
    let options = ProofOptions::new(
        28,
        8,
        0,
        FieldExtension::None,
        4,
        31,
        BatchingMethod::Linear,
        BatchingMethod::Linear,
    );
    
    let prover = FibLikeProver::<winterfell::crypto::hashers::Blake3_256<BaseElement>>::new(options);
    
    let trace = prover.build_trace(num_steps);
    let pub_inputs = prover.get_pub_inputs(&trace);
    
    println!("Final result: {}", pub_inputs);
    
    let proof = prover.prove(trace)?;
    println!("Proof generated successfully!");
    
    let acceptable_options = winterfell::AcceptableOptions::OptionSet(vec![proof.options().clone()]);
    
    match winterfell::verify::<FibLikeAir, winterfell::crypto::hashers::Blake3_256<BaseElement>, DefaultRandomCoin<winterfell::crypto::hashers::Blake3_256<BaseElement>>, MerkleTree<winterfell::crypto::hashers::Blake3_256<BaseElement>>>(
        proof,
        pub_inputs,
        &acceptable_options,
    ) {
        Ok(()) => println!("Proof verified successfully!"),
        Err(e) => println!("Proof verification failed: {:?}", e),
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_like_sequence() {
        run_example(8).unwrap();
    }
}
