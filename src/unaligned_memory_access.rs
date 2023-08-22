#[repr(packed)]
#[derive(Clone, Copy)]
struct FooUnaligned {
    _byte: u8,
    quan: usize,
}

#[derive(Clone, Copy)]
struct FooAligned {
    _byte: u8,
    quan: usize,
}

pub fn aligned_access() {
    let foo = FooAligned {
        _byte: 0,
        quan: 0,
    };
    let mut arr = vec![foo; 10000];
    for f in &mut arr {
        f.quan += 1;
    }
}

pub fn unaligned_access() {
    let foo = FooUnaligned {
        _byte: 0,
        quan: 0,
    };
    let mut arr = vec![foo; 10000];
    for f in &mut arr {
        f.quan += 1;
    }
}
