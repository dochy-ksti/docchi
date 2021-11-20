use std::hash::Hasher;
use metrohash::MetroHash128;
use std::sync::{Arc};
use std::sync::mpsc::{Sender, channel, Receiver};
use crate::ArcResult;

pub(crate) struct HashThread{
    vec_sender : Sender<Option<(String, Arc<Vec<u8>>)>>,
    result_receiver : Receiver<ArcResult<u128>>,
}

impl HashThread{
    pub fn new() -> HashThread{
        let (vec_sender, vec_receiver) = channel();
        let (result_sender, result_receiver) = channel();
        std::thread::spawn(move || {
            let mut hasher = MetroHash128::new();
            loop{
                match vec_receiver.recv(){
                    Ok(Some((path, v))) =>{
                        let path : String = path;
                        let v : Arc<Vec<u8>> = v;
                        hasher.write(path.as_bytes());
                        hasher.write(v.as_ref())
                    },
                    Ok(None) => break,
                    Err(e) =>{ result_sender.send(Err(e.into())).ok(); return; }
                }
            }
            let (l,r) = hasher.finish128();
            result_sender.send(Ok(to_u128(l,r))).ok();
        });
        HashThread{
            vec_sender,
            result_receiver,
        }
    }

    pub fn calc_hash(&mut self, path : String, vec : Arc<Vec<u8>>){
        self.vec_sender.send(Some((path, vec))).ok();
    }

    pub fn finish(&self) -> ArcResult<u128>{
        self.vec_sender.send(None).ok();
        Ok(self.result_receiver.recv()??)
    }
}

fn to_u128(l : u64, r : u64) -> u128{
    let mut r = u128::from(r);
    r |= (l as u128) << 64;
    r
}