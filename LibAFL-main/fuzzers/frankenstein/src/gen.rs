#[derive(Clone, Debug)]
/// Generates random bytes
pub struct RandEIRGenerator<S>
where
    S: HasRand,
{
    max_size: usize,
    phantom: PhantomData<S>,
}

impl<S> Generator<BytesInput, S> for RandEIRGenerator<S>
where
    S: HasRand,
{
    fn generate(&mut self, state: &mut S) -> Result<BytesInput, Error> {
        let mut size = state.rand_mut().below(self.max_size as u64);
        if size == 0 {
            size = 1;
        }
        let random_bytes: Vec<u8> = (0..size)
            .map(|_| state.rand_mut().below(256) as u8)
            .collect();
        Ok(BytesInput::new(random_bytes))
    }
}

impl<S> RandEIRGenerator<S>
where
    S: HasRand,
{
    /// Returns a new [`RandEIRGenerator`], generating up to `max_size` random bytes.
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            phantom: PhantomData,
        }
    }
}