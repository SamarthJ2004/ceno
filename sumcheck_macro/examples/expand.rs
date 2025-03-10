/// To see the code generated by the macro, run the following command in the sumcheck_macro directory:
/// ```sh
/// cargo expand --example expand
/// ```
use ff_ext::ExtensionField;
use ff_ext::GoldilocksExt2;
use multilinear_extensions::{
    mle::FieldType, util::largest_even_below, virtual_poly::VirtualPolynomial,
};
use p3_field::PrimeCharacteristicRing;
use sumcheck::util::{AdditiveArray, ceil_log2};

#[derive(Default)]
struct Container<'a, E: ExtensionField> {
    poly: VirtualPolynomial<'a, E>,
    round: usize,
}

fn main() {
    let c = Container::<GoldilocksExt2>::default();
    c.run();
}

impl<E: ExtensionField> Container<'_, E> {
    pub fn run(&self) {
        let _result: AdditiveArray<_, 4> =
            sumcheck_macro::sumcheck_code_gen!(3, false, |_| &self.poly.flattened_ml_extensions[0]);
    }
}
