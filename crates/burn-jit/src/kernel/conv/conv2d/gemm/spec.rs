use cubecl::prelude::Numeric;
use std::marker::PhantomData;

/// Implicit convolution spec definiting each element types used in the computation.
pub trait ConvSpec: Send + Sync + Clone + 'static {
    /// Element type of each input and output tensor of the kernel.
    type EG: Numeric;
    /// Element type of the intermediate representation of the inputs.
    type ES: Numeric;
    /// Element type of the intermediate representation of the output accumulator.
    type EA: Numeric;
}

/// Specification for a single conv using global tensor as inputs.
#[derive(Clone)]
pub struct SingleConvSpec<EG: Numeric, ES: Numeric, EA: Numeric> {
    _eg: PhantomData<EG>,
    _es: PhantomData<ES>,
    _ea: PhantomData<EA>,
}

impl<EG: Numeric, ES: Numeric, EA: Numeric> ConvSpec for SingleConvSpec<EG, ES, EA> {
    type EG = EG;
    type ES = ES;
    type EA = EA;
}
