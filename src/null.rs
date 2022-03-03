use aper::{APerElement, Constraints, DecodeError, Decoder, EncodeError, Encoding};

impl APerElement for () {
    /// Read `()` from an aligned PER encoding.
    fn from_aper(_: &mut Decoder, _: Constraints) -> Result<Self, DecodeError> {
        Ok(())
    }

    fn to_aper(&self, _: Constraints) -> Result<Encoding, EncodeError> {
        Ok(Encoding::new())
    }
}
