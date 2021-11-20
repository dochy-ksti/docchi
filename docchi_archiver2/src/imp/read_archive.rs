use std::io::Read;
use crate::{ArchiveData, ArcResult};
use with_capacity_safe::vec_with_capacity_safe;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::imp::structs::archiver::Archiver;




pub fn read_archive<R : Read, T : Send + 'static>(converter : impl Fn(&str, &[u8]) -> T + Send + Sync + 'static, r : &mut R) -> ArcResult<ArchiveData<T>> {
    let (kvals, _) = docchi_compaction::decode(r)?;

    let mut iter = kvals.into_iter();
    let len = iter.next().ok_or("Found no files")?.as_i64().ok_or("First value must be Int")? as usize;
    let mut raw_senders: Vec<(usize, Sender<ArcResult<Vec<u8>>>)> = vec_with_capacity_safe(len)?;
    let mut raw_receivers: Vec<(String, Receiver<ArcResult<Vec<u8>>>)> = vec_with_capacity_safe(len)?;

    for _ in 0..len {
        let path = iter.next().ok_or("Parse Error")?.as_string().ok_or("Parse Error")?;
        let len = iter.next().ok_or("Parse Error")?.as_i64().ok_or("Parse Error")? as usize;

        let (raw_sender, raw_receiver) = channel();
        raw_senders.push((len, raw_sender));
        raw_receivers.push((path, raw_receiver));
    }

    let (archive_sender, archive_receiver) = channel();

    std::thread::spawn(move ||{
        let mut archiver = Archiver::new(converter);
        for (path, data) in raw_receivers{
            match data.recv(){
                Ok(Ok(v)) =>{ archiver.archive(path, v) },
                Ok(Err(e)) =>{
                    archive_sender.send(Err(e)).ok();
                    return;
                }
                Err(e) => {
                    archive_sender.send(Err(e.into())).ok();
                    return;
                }
            }
        }
        archive_sender.send(archiver.finish()).ok();
    });
    for (len, raw_sender) in raw_senders {
        let mut buf = vec_with_capacity_safe(len)?;
        unsafe{ buf.set_len(len); }

        r.read_exact(&mut buf)?;
        rayon::spawn_fifo(move ||{
            let mut decoder = snap::raw::Decoder::new();
            match decoder.decompress_vec(&buf) {
                Ok(v) => raw_sender.send(Ok(v)).ok(),
                Err(e) => raw_sender.send(Err(e.into())).ok(),
            };
        });
    }


    archive_receiver.recv()?
}