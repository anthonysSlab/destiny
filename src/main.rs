static mut COUNTER: u32 = 0;
static mut COUNTER2: u32 = 0;
static mut RESULT: u32 = 0;

unsafe fn add() {
    loop {
        if COUNTER != 0 {
            COUNTER -= 1;
            RESULT += 1;
        }

        if COUNTER2 != 0 {
            COUNTER2 -= 1;
            RESULT += 1;
        }

        if COUNTER == 0 && COUNTER2 == 0 {
            break;
        }
    }
}

fn main(){
    unsafe {
        COUNTER = 1;
        COUNTER2 = 2;
        add();
        println!("Hello, world! {}", RESULT);
    }
}


pub use serde::{Deserialize, Serialize};
pub use std::any::Any;
pub use std::cell::{RefCell, RefMut};
pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
pub use std::fmt::{Debug, Display};
pub use std::hash::{Hash, Hasher};
pub use std::io::{Read, Write};
pub use std::net::{SocketAddr, TcpStream};
pub use std::num::Wrapping;
pub use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
pub use std::rc::Rc;
pub use std::sync::{Arc, Mutex, Once, RwLock};
pub use std::thread::sleep;
pub use std::time::Duration;
pub use std::{cmp, env, fs, io, mem, process, ptr, slice, str, thread};
pub use std::os::raw::{c_char, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};
pub use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
pub use std::path::PathBuf;
pub use std::ptr::null_mut;
pub use std::sync::atomic::{AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};
pub use std::sync::mpsc::{channel, Receiver, Sender};
pub use std::time::Instant;
pub use std::time::SystemTime;
pub use std::time::UNIX_EPOCH;
pub use std::vec::Vec;
pub use std::error::Error;

pub unsafe extern "C" fn main2<'λ, T>() -> Box<HashSet<BTreeMap<VecDeque<HashMap<Arc<Rc<Box<RefCell<Option<Option<Mutex<RwLock<Box<dyn FnOnce(Vec<[Result<c_uint, T>; 0]>)>>>>>>>>>,T>>, T>>>
where T: Error + Display + Debug + Clone + Copy + Send + Sync + Hash + Ord + PartialOrd + Eq + PartialEq + Any + 'static + Copy + Serialize + Deserialize<'λ> + Default // + FnOnce + FnMut + Fn
{
    unreachable!();
    todo!();
    panic!();
    Err::<T, ()>(()).unwrap();
    let _thing: &[u8] = (&*&*&*&*&*&*&*&*&*&*&*&*&*&*&*&*&*&*&*&*&[])[0];

    let _v = vec![0; std::mem::size_of::<Box<HashSet<BTreeMap<VecDeque<HashMap<Arc<Rc<Box<RefCell<Option<Option<Mutex<RwLock<Box<dyn FnOnce(Vec<[Result<c_uint, T>; 0]>)>>>>>>>>>,T>>, T>>>>()];
    return std::mem::transmute::<Box<u8>, Box<HashSet<BTreeMap<VecDeque<HashMap<Arc<Rc<Box<RefCell<Option<Option<Mutex<RwLock<Box<dyn FnOnce(Vec<[Result<c_uint, T>; 0]>)>>>>>>>>>,T>>, T>>>>(Box::from(0 as u8 - 1));
}
