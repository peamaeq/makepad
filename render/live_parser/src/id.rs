#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Once;
use std::fmt;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialOrd, Hash, PartialEq)]
pub struct LiveFileId(u16);

impl LiveFileId{
    pub fn index(index:usize)->LiveFileId{LiveFileId(index as u16)}
    pub fn to_index(&self) -> usize{self.0 as usize}
}

#[derive(Clone, Eq, Hash, Copy, PartialEq)]
pub struct Id(pub u64);

#[derive(Clone, Debug, Eq, Hash, Copy, PartialEq)]
pub struct LiveNodePtr {
    pub level: usize,
    pub index: usize
}


#[derive(Debug)]
pub enum IdType {
    Empty,
    Multi {index: usize, count: usize},
    NodePtr {file_id: LiveFileId, ptr: LiveNodePtr},
    Single(u64),
    Number(u64)
}

// Id uses the high 3 bits to signal type
// 0?? = Single Id    0x8 == 0
// 100 = Empty     0x8
// 101 = NodePtr   0xA
// 110 = Multi     0xC
// 111 = Number    0xE

impl Id {
    /*
    // ok fine maybe this one was too simple.
    pub const fn from_str(idstr: &str) -> Id {
        let id = idstr.as_bytes();
        let id_len = id.len();
        let mut ret = 0u64;
        let mut o = 0;
        let mut i = 0;
        while i < id_len {
            ret ^= (id[i] as u64) << ((o & 7) << 3);
            o += 1;
            i += 1;
        }
        return Id(ret & 0x7fff_ffff_ffff_ffff)
    }*/
    
    // from https://nullprogram.com/blog/2018/07/31/
    // i have no idea what im doing with start value and finalisation.
    pub const fn from_str(idstr: &str) -> Id {
        let id = idstr.as_bytes();
        let id_len = id.len();
        let mut x = 0xd6e8_feb8_6659_fd9u64;
        let mut i = 0;
        while i < id_len {
            x = x.overflowing_add(id[i] as u64).0;
            x ^= x >> 32;
            x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
            x ^= x >> 32;
            x = x.overflowing_mul(0xd6e8_feb8_6659_fd93).0;
            x ^= x >> 32;
            i += 1;
        }
        return Id(x & 0x7fff_ffff_ffff_ffff) // leave the first bit
    }
    
    pub fn to_type(&self) -> IdType {
        if self.0 & 0x8000_0000_0000_0000 != 0 {
            match self.0 & 0xE000_0000_0000_0000 {
                0xA000_0000_0000_0000 => IdType::NodePtr {
                    file_id: LiveFileId(((self.0 >> 32) & 0xffff) as u16),
                    ptr: LiveNodePtr {
                        level: ((self.0 >> 48) & 0x1fff) as usize,
                        index: (self.0 & 0xffff_ffff)as usize
                    }
                },
                0xC000_0000_0000_0000 => IdType::Multi {
                    index: (self.0 & 0xffff_ffff) as usize,
                    count: ((self.0 & 0x1fff_ffff_ffff_ffff) >> 32) as usize,
                },
                0xE000_0000_0000_0000 => IdType::Number(self.0 & 0x1fff_ffff_ffff_ffff),
                _ => IdType::Empty
            }
        }
        else {
            IdType::Single(self.0 & 0x7fff_ffff_ffff_ffff)
        }
    }
    
    pub fn multi(index: usize, len: usize) -> Id {
        Id(((((len as u64) << 32) | index as u64) & 0x1fff_ffff_ffff_ffff) | 0xC000_0000_0000_0000)
    }
    
    pub fn single(val: u64) -> Id {
        Id(val & 0x7fff_ffff_ffff_ffff)
    }
    
    pub fn number(val: u64) -> Id {
        Id(0xE000_0000_0000_0000 | (val & 0x1fff_ffff_ffff_ffff))
    }
    
    pub fn empty() -> Id {
        Id(0x8000_0000_0000_0000)
    }
    
    pub fn node_ptr(file_id: LiveFileId, ptr: LiveNodePtr)->Id{
        Id(
            0xA000_0000_0000_0000 |
            (ptr.index as u64) |
            ((file_id.0 as u64) << 32) |
            ((ptr.level as u64) << 48) 
        )
    }
    
    pub fn is_empty(&self) -> bool {
        self.0 & 0xE000_0000_0000_0000 == 0x8000_0000_0000_0000
    }
    
    pub fn is_node_ptr(&self) -> bool {
        self.0 & 0xE000_0000_0000_0000 == 0xA000_0000_0000_0000
    }
        
    pub fn is_multi(&self) -> bool {
        self.0 & 0xE000_0000_0000_0000 == 0xC000_0000_0000_0000
    }
    
    pub fn is_number(&self) -> bool {
        self.0 & 0xE000_0000_0000_0000 == 0xE000_0000_0000_0000
    }
    
    pub fn is_single(&self) -> bool {
        (self.0 & 0x8000_0000_0000_0000) == 0
    }
    
    pub fn get_multi(&self) -> (usize, usize) {
        if !self.is_multi() {
            panic!()
        }
        (
            (self.0 & 0xffff_ffff) as usize,
            ((self.0 & 0x1fff_ffff_ffff_ffff) >> 32) as usize,
        )
    }
    
    pub fn get_single(&self) -> u64 {
        if !self.is_single() {
            panic!()
        }
        self.0
    }
    
    pub fn panic_collision(self, val: &str) -> Id {
        if let Some(s) = self.check_collision(val) {
            panic!("Collision {} {}", val, s)
        }
        self
    }
    
    pub fn check_collision(&self, val: &str) -> Option<String> {
        IdMap::with( | idmap | {
            if self.is_single() {
                if let Some(stored) = idmap.id_to_string.get(self) {
                    if stored != val {
                        return Some(stored.clone())
                    }
                }
                else {
                    idmap.id_to_string.insert(self.clone(), val.to_string());
                }
            }
            return None;
        })
    }
    
    fn as_string<F, R>(&self, f: F) -> R
    where F: FnOnce(Option<&String>) -> R
    {
        IdMap::with( | idmap | {
            if self.is_single() {
                return f(idmap.id_to_string.get(self))
            }
            return f(None);
        })
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_type() {
            IdType::Multi {index, count} => {
                write!(f, "MultiId {} {}", index, count)
            },
            IdType::Single(_) => {
                self.as_string( | string | {
                    if let Some(id) = string {
                        write!(f, "{}", id)
                    }
                    else {
                        write!(f, "IdNotFound {:x}", self.0)
                    }
                })
            },
            IdType::Number(value) => {
                write!(f, "{}", value)
            },
            IdType::Empty => {
                write!(f, "IdEmpty")
            }
            IdType::NodePtr{file_id, ptr}=>{
                write!(f, "NodePtr{{file:{}, level:{}, index:{}}}", file_id.0, ptr.level, ptr.index)
            }
        }
        
    }
}


impl fmt::LowerHex for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}


pub struct IdMap {
    id_to_string: HashMap<Id, String>,
}

impl IdMap {
    pub fn add(&mut self, val: &str) {
        self.id_to_string.insert(Id::from_str(val), val.to_string());
    }
    
    pub fn contains(&mut self, val: &str) -> bool {
        self.id_to_string.contains_key(&Id::from_str(val))
    }
    
    pub fn with<F, R>(f: F) -> R
    where
    F: FnOnce(&mut IdMap) -> R,
    {
        static mut IDMAP: Option<IdMap> = None;
        static ONCE: Once = Once::new();
        ONCE.call_once( || unsafe {
            IDMAP = Some(IdMap {
                id_to_string: HashMap::new()
            })
        });
        f(unsafe {IDMAP.as_mut().unwrap()})
    }
}


pub struct IdFmt<'a> {
    multi_ids: &'a Vec<Id>,
    is_dot: bool,
    id: Id
}

impl <'a> IdFmt<'a> {
    pub fn dot(multi_ids: &'a Vec<Id>, id: Id) -> Self {
        Self {multi_ids, is_dot: true, id}
    }
    pub fn col(multi_ids: &'a Vec<Id>, id: Id) -> Self {
        Self {multi_ids, is_dot: false, id}
    }
}

impl <'a> fmt::Display for IdFmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.id.to_type() {
            IdType::Multi {index, count} => {
                for i in 0..count {
                    let _ = write!(f, "{}", self.multi_ids[(i + index) as usize]);
                    if i < count - 1 {
                        if self.is_dot {
                            let _ = write!(f, ".");
                        }
                        else {
                            let _ = write!(f, "::");
                        }
                    }
                }
                fmt::Result::Ok(())
            },
            _ => {
                write!(f, "{}", self.id)
            },
        }
    }
}

