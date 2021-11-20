use crate::kval_enum::{KVal, Decimal};
use crate::{encode, decode};
use anyhow::Result;
use crate::enc_dec::encode_to_vec::encode_to_vec;


#[test]
fn test2() -> Result<()>{
    fn decimal(int : i128, dot : u8) -> KVal {
        KVal::Decimal(Decimal::new(int, dot))
    }
    let vec : Vec<KVal> = vec![
        KVal::Null,
        KVal::Bit(false),
        KVal::Bool(true),
        KVal::Byte(0),
        KVal::Byte(20),
        KVal::Byte(-128),
        KVal::Str16("amenbou".to_string()),
        KVal::Str16("".to_string()),
        KVal::Str16("012345678901234".to_string()),
        KVal::Int(0),
        KVal::Int(-50),
        KVal::Int(200),
        KVal::Int(-200),
        KVal::Int(32767),
        KVal::Int(-32767),
        KVal::Int(-32768),
        KVal::Int(32768),
        KVal::Float(0.1f32),
        KVal::Str256("01234567890123456".to_string()),
        KVal::Str256("".to_string()),
        KVal::Str256("01234".to_string()),
        KVal::Double(0.1f64),
        decimal(0, 0),
        decimal(0, 4),
        decimal(65536*65536*65536, 4),
        decimal(-65536*65536*65536, 4),
        decimal(65536*65536*65536*65536, 0),
        decimal(-65536*65536*65536*65536, 0),
        decimal(65536*65536*65536*65536 / 2, 0),
        decimal(-65536*65536*65536*65536 / 2, 0),
        decimal(65536*65536*65536*65536 / 4, 0),
        decimal(-65536*65536*65536*65536 / 4, 0),
        decimal(-65536*65536*65536*65536*65536, 0),
        decimal(65536*65536*65536*65536*65536 / 2, 0),
        decimal(-65536*65536*65536*65536*65536 / 2, 0),
        decimal(65536*65536*65536*65536*65536 / 4, 0),
        decimal(-65536*65536*65536*65536*65536 / 4, 0),
        decimal(65536*65536*65536*65536*65536, 0),

        KVal::BigStr(String::from_utf8(vec!['a' as u8; 256]).unwrap()),
        KVal::BigStr("".to_string()),
        KVal::Binary(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17]),
        KVal::Binary(vec![127;127]),
        KVal::Binary(vec![128;128]),
        KVal::Binary(vec![129;255]),
        KVal::Binary(vec![130;256]),
        KVal::Binary(vec![]),
        KVal::Binary8(vec![0,u32::MAX as u64 + 1, u64::MAX]),
        KVal::Binary8(vec![]),
        KVal::Binary4(vec![0,u16::MAX as u32 + 1, u32::MAX]),
        KVal::Binary4(vec![]),
        KVal::Binary2(vec![0,u8::MAX as u16 + 1, u16::MAX]),
        KVal::Binary2(vec![]),
        KVal::Binary(vec![]),
        KVal::Undefined(0),
        KVal::Undefined(7),
    ];

    let encoded = encode_to_vec(&vec)?;
    let (decoded, size) = decode(&mut encoded.as_slice())?;
    assert_eq!(encoded.len(), size);
    assert_eq!(vec, decoded);
    Ok(())
}