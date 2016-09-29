use aper::{APerElement, Constraint, Constraints, Decoder, DecodeError};

impl<T: APerElement> APerElement for Vec<T> {
    type Result = Vec<T::Result>;
    const TAG: u32 = 0xBEEF;
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: None,
    };

    fn aper_decode(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, DecodeError> {
        if constraints.size.is_none() {
            return Err(DecodeError::Dummy); // XXX: meaningful error here
        }
        let sz_constr = constraints.size.unwrap();

        let mut min_len: usize = 0;
        let mut max_len: usize = 0;
        if sz_constr.min().is_some() {
            min_len = sz_constr.min().unwrap() as usize;
        }
        if sz_constr.max().is_some() {
            max_len = sz_constr.max().unwrap() as usize;
        }

        if max_len >= 65535 {
            unimplemented!();
        }

        let mut len: usize = 0;
        if max_len == min_len {
            len = max_len;
        } else {
            let ret = decoder.decode_length();
            if ret.is_err() {
                return Err(DecodeError::Dummy); // XXX: meaningful error code
            }
            len = ret.unwrap();
        }

        // XXX: This is terrible, but convenient. Either fix or document thoroughly.
        let mut el_constrs = Constraints {
            value: None,
            size: constraints.value,
        };
        let mut content: Vec<T::Result> = Vec::with_capacity(len);
        for _ in 0..len {
            let ret = decoder.decode::<T>(el_constrs);
            if ret.is_err() {
                return Err(DecodeError::Dummy); // XXX: meaningful error here
            }
            content.push(ret.unwrap());
        }

        Ok(content)
    }
}
