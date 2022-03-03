extern crate asn1;
use asn1::aper::{self, APerElement, Constraint, Constraints, Encoding, UNCONSTRAINED};
use asn1::BitString;

#[derive(Debug)]
struct Foo {
    pub foo: BitString,
    pub bar: Vec<u8>,
    pub baz: Vec<BitString>,
}

impl APerElement for Foo {
    fn from_aper(decoder: &mut aper::Decoder, _: Constraints) -> Result<Self, aper::DecodeError> {
        let foo = BitString::from_aper(
            decoder,
            Constraints {
                value: None,
                size: Some(Constraint::new(None, Some(4))),
            },
        );

        let bar = Vec::<u8>::from_aper(
            decoder,
            Constraints {
                value: None,
                size: Some(Constraint::new(None, Some(3))),
            },
        );

        let baz = Vec::<BitString>::from_aper(
            decoder,
            Constraints {
                // here the "value" constraint is a constraint on the size of each element
                value: Some(Constraint::new(None, Some(4))),
                // "size" behaves normally
                size: Some(Constraint::new(None, Some(2))),
            },
        );

        if foo.is_err() {
            return Err(foo.err().unwrap());
        }
        if bar.is_err() {
            return Err(bar.err().unwrap());
        }
        if baz.is_err() {
            return Err(baz.err().unwrap());
        }

        Ok(Foo {
            foo: foo.unwrap(),
            bar: bar.unwrap(),
            baz: baz.unwrap(),
        })
    }

    fn to_aper(&self, _: Constraints) -> Result<Encoding, aper::EncodeError> {
        let mut enc = self
            .foo
            .to_aper(Constraints {
                value: None,
                size: Some(Constraint::new(None, Some(4))),
            })
            .unwrap();

        enc.append(
            &self
                .bar
                .to_aper(Constraints {
                    value: None,
                    size: Some(Constraint::new(None, Some(3))),
                })
                .unwrap(),
        )
        .unwrap();

        enc.append(
            &self
                .baz
                .to_aper(Constraints {
                    // here the "value" constraint is a constraint on the size of each element
                    value: Some(Constraint::new(None, Some(4))),
                    // "size" behaves normally
                    size: Some(Constraint::new(None, Some(2))),
                })
                .unwrap(),
        )
        .unwrap();

        Ok(enc)
    }
}

#[test]
fn encode_foo() {
    let x = Foo {
        foo: BitString::with_bytes_and_len(&vec![0x0e], 4),
        bar: vec![0x46, 0x4f, 0x4f],
        baz: vec![
            BitString::with_bytes_and_len(&vec![0x0e], 4),
            BitString::with_bytes_and_len(&vec![0x0e], 4),
        ],
    };
    let target: Vec<u8> = vec![0xe0, 0x34, 0x64, 0xf4, 0xf0, 0x2e, 0xe0];
    assert_eq!(target, *x.to_aper(UNCONSTRAINED).unwrap().bytes());
}

#[test]
fn decode_foo() {
    // [14, 3, 70, 79, 79, 2, 224, 224]
    let data = b"\x0e\x03\x46\x4f\x4f\x02\xee";
    let mut d = aper::Decoder::new(data);
    d.read(4).unwrap(); // strip left-padding
    let f = Foo::from_aper(&mut d, UNCONSTRAINED).unwrap();
    let target_bar = vec![0x46 as u8, 0x4f as u8, 0x4f as u8];

    for i in 0..4 {
        if i == 1 || i == 2 || i == 3 {
            assert_eq!(true, f.foo.is_set(i));
        } else {
            assert_eq!(false, f.foo.is_set(i));
        }
    }

    assert_eq!(f.bar.len(), target_bar.len());
    for i in 0..f.bar.len() {
        assert_eq!(f.bar[i], target_bar[i]);
    }

    assert_eq!(f.baz.len(), 2);
    for i in 0..f.baz.len() {
        for j in 0..4 {
            if j == 1 || j == 2 || j == 3 {
                assert_eq!(true, f.baz[i].is_set(j));
            } else {
                assert_eq!(false, f.baz[i].is_set(j));
            }
        }
    }
}
