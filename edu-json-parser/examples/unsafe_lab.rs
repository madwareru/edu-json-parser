use std::{str};
use std::collections::HashMap;

pub struct StringAllocator {
    buf: String,
    prev_buffers: Vec<String>,
    current_offset: usize
}

impl StringAllocator {
    pub fn new() -> Self {
        StringAllocator {
            buf: String::with_capacity(4096),
            prev_buffers: Vec::new(),
            current_offset: 0
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        StringAllocator {
            buf: String::with_capacity(cap),
            prev_buffers: Vec::new(),
            current_offset: 0
        }
    }

    // You should be 100 percent sure that f(&mut s) will not grow capacity of an s.
    // In other case, you are in a situation of a panic.
    // So, be sure that certain_len_in_bytes calculated right
    // It could be higher than actually needed length anyway,
    // but in this case you might take non optimal memory consumption
    pub unsafe fn allocate_string<F>(&mut self, certain_len_in_bytes: usize, mut f: F) -> &'static str
        where F: FnMut(&mut String) -> ()
    {
        if self.current_offset + certain_len_in_bytes > self.buf.capacity() {
            let mut new_capacity = self.buf.capacity() * 2;
            while self.current_offset + certain_len_in_bytes > new_capacity {
                new_capacity *= 2;
            }
            let old_buffer = std::mem::replace(&mut self.buf, String::with_capacity(new_capacity));
            self.prev_buffers.push(old_buffer);
            self.current_offset = 0;
        }
        let str_slice = {
            let start = self.current_offset;
            let old_cap = self.buf.capacity();
            f(&mut self.buf);
            if self.buf.len() - self.current_offset > certain_len_in_bytes {
                panic!("certain_len_in_bytes is lower than it should be!")
            }
            if self.buf.capacity() > old_cap {
                panic!("unexpected grow of capacity")
            }
            &self.buf[start..self.buf.len()]
        };
        self.current_offset = self.buf.len();
        &*(str_slice as *const str)
    }
}

pub struct Interner {
    map: HashMap<&'static str, u32>,
    vec: Vec<&'static str>,
    allocator: StringAllocator
}

impl Interner {
    pub fn with_capacity(cap: usize) -> Interner {
        let cap = cap.next_power_of_two();
        Interner {
            map: HashMap::default(),
            vec: Vec::new(),
            allocator: StringAllocator::with_capacity(cap)
        }
    }

    pub fn intern(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.map.get(name) {
            return id;
        }
        let name = unsafe {
            self.allocator.allocate_string(name.len(), |s: &mut String| s.push_str(name))
        };
        let id = self.map.len() as u32;
        self.map.insert(name, id);
        self.vec.push(name);

        debug_assert!(self.lookup(id) == name);
        debug_assert!(self.intern(name) == id);

        id
    }

    pub fn lookup(&self, id: u32) -> &str {
        self.vec[id as usize]
    }
}

const HELLO: &'static str = "Hello, ";
const WORLD: &'static str = "World!";

fn main() {
    unsafe {
        let mut pretender_allocator = StringAllocator::with_capacity(16);
        let certain_size = HELLO.len() + WORLD.len();
        let hw = pretender_allocator.allocate_string(certain_size, |s: &mut String|{
            s.push_str(HELLO);
            s.push_str(WORLD);
        });
        let why_are_you_doing_this = pretender_allocator.allocate_string(32, |s: &mut String|{
            s.push_str("to be or not to be?");
        });
        let why_are_you_doing_this2 = pretender_allocator.allocate_string(48, |s: &mut String|{
            s.push_str("lorum ipsum dolor sit amet ktulhu ftagn");
        });
        println!("String content: {}", hw);
        println!("String content: {}", why_are_you_doing_this);
        println!("String content: {}", why_are_you_doing_this2);
    }

    let mut interner = Interner::with_capacity(32);
    let for_interned = interner.intern("for");
    let while_interned = interner.intern("while");
    let repeat_interned = interner.intern("repeat");
    let until_interned = interner.intern("until");
    let reinterpret_interned = interner.intern("reinterpret");
    let cast_interned = interner.intern("cast");
    let repeat_duplicate = interner.intern("repeat");

    println!(
        "Interners are: {}, {}, {}, {}, {}, {}, {}",
        for_interned,
        while_interned,
        repeat_interned,
        until_interned,
        reinterpret_interned,
        cast_interned,
        repeat_duplicate
    );

}