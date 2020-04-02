use std::{str, slice};
use std::ops::Deref;

pub struct StringPretender {
    len: usize,
    internal_ptr: *const u8
}

impl StringPretender {
    pub fn from_raw_ptr_and_len(ptr: *const u8, len: usize) -> Self {
        StringPretender{ internal_ptr: ptr, len }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(
                slice::from_raw_parts(self.internal_ptr, self.len)
            )
        }
    }
}

impl Deref for StringPretender {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

pub struct StringPretenderAllocator {
    buf: String,
    prev_buffers: Vec<String>,
    current_offset: usize
}

impl StringPretenderAllocator {
    pub fn new() -> Self {
        StringPretenderAllocator {
            buf: String::with_capacity(4096),
            prev_buffers: Vec::new(),
            current_offset: 0
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        StringPretenderAllocator {
            buf: String::with_capacity(cap),
            prev_buffers: Vec::new(),
            current_offset: 0
        }
    }

    // You should be 100 percent sure that f(&mut s) will not grow capacity of an s.
    // In other case, you are in a situation of a panic.
    // So, be sure that certain_len_in_bytes calculated right
    pub fn allocate_string<F>(&mut self, certain_len_in_bytes: usize, mut f: F) -> StringPretender
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
        let old_cap = self.buf.capacity();
        f(&mut self.buf);
        if self.buf.len() - self.current_offset > certain_len_in_bytes {
            panic!("certain_len_in_bytes is lower than it should be!")
        }
        if self.buf.capacity() > old_cap {
            panic!("unexpected grow of capacity")
        }
        let pretender = unsafe {
            let ptr = self.buf.as_mut_ptr();
            let part: *mut u8 = ptr.add(self.current_offset);
            StringPretender::from_raw_ptr_and_len(part, self.buf.len() - self.current_offset)
        };
        self.current_offset = self.buf.len();
        pretender
    }
}

const HELLO: &'static str = "Hello, ";
const WORLD: &'static str = "World!";

fn main() {
    let mut pretender_allocator = StringPretenderAllocator::with_capacity(16);
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
    println!("String content: {}", hw.deref());
    println!("String content: {}", why_are_you_doing_this.deref());
    println!("String content: {}", why_are_you_doing_this2.deref());
}