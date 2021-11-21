A serializer which serializes values with dynamic types and compresses integers, bits, 
and strings which represents decimals. 

Even if a variable's size is 8 byte, when the actual value is 0 or 1, 
this library can compress it to two bits.

This is a base component of the Docchi File System.

```
pub struct SomeCollection{
    vec : Vec<SomeItem>
}

impl SomeCollection{
    pub fn encode<W : std::io::Write>(&self, write : &mut W) -> Result<()>{
    
        //KVal is the value type of this crate
        let mut vec : Vec<docchi_compaction::KVal> = vec![];

        //comp_int compresses integers considering actual size. 0,1 -> Bit. -128 to 127 -> Byte...
        vec.push(comp_int(self.len() as i64));  
        
        for item in self.vec.iter(){
            item.write(&mut vec);
        }

        //encode values to bytes
        docchi_compaction::encode(&vec, write)?;
        Ok(())
    }
    
    pub fn decode<R : std::io::Read>(read : &mut R) -> Result<Self>{
        //decode to Vec<KVal>
        let (kvals,_) = docchi_compaction::decode(read)?;
        
        let mut iter = kvals.iter();
        let len = iter.next()?.as_i64()? as usize;
        
        let mut vec : Vec<SomeItem> = Vec::with_capacity(len);

        for _ in 0..len{
            vec.push(SomeItem::read(&mut iter)?)
        }

        Ok(SomeCollection{ vec })
    }
}

pub struct SomeItem{
    index : usize,
    value : u8,
}

impl SomeItem{
    pub fn write(&self, vec : &mut Vec<KVal>){
        vec.push(comp_int(self.index as i64));  
        vec.push(comp_int(self.value as i64));
    }
    pub fn read<'a>(iter : &mut impl Iterator<Item=&'a KVal>) -> Result<SomeItem>{ //This Result implements From<NoneError>
        let index = iter.next()?.as_i64()? as usize; 
        let value = iter.next()?.as_i64()? as u8;

        Ok(SomeItem{ index, value })
    }
}
```

### changelog
#### 0.4.0 

change encode of the values "undefined".

add Binary, Binary8, Binary4, and Binary2 types.