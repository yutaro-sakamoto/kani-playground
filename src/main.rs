fn estimate_size(x: u32) -> u32 {
    assert!(x < 4096);

    if x < 256 {
        if x < 128 {
            return 1;
        } else {
            return 3;
        }
    } else if x < 1024 {
        if x > 1022 {
            return 4;
        } else {
            return 5;
        }
    } else {
        if x < 2048 {
            return 7;
        } else {
            return 9;
        }
    }
}

//#[cfg(kani)]
//#[kani::proof]
//fn check_estimate_size() {
//    let x: u32 = kani::any();
//    kani::assume(x < 4096);
//    let y = estimate_size(x);
//    assert!(y < 10);
//}

fn get_wrapped(i: usize, a: &[u32]) -> u32 {
    if a.len() == 0 {
        return 0;
    }
    return a[(i % a.len() + 1) % a.len()];
}

fn simple_addition(a: u32, b: u32) -> u32 {
    a + b
}

//#[cfg(kani)]
//#[kani::proof]
//fn add_overflow() {
//    let a: u32 = kani::any();
//    let b: u32 = kani::any();
//    simple_addition(a, b);
//}

fn find_midpoint(low: u32, high: u32) -> u32 {
    let low_u64 = low as u64;
    let high_u64 = high as u64;
    ((low_u64 + high_u64) / 2) as u32
}

//#[cfg(kani)]
//#[kani::proof]
//fn midpoint_overflow() {
//    let a = kani::any();
//    let b = kani::any();
//    //kani::assume(a < b);
//    find_midpoint(a, b);
//}

fn initialize_prefix(length: usize, buffer: &mut [u8]) {
    if length > buffer.len() {
        return;
    }

    for i in 0..length {
        buffer[i] = 0;
    }
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(11)]
fn check_initialize_prefix() {
    const LIMIT: usize = 10;
    let mut buffer: [u8; LIMIT] = [1; LIMIT];

    let length = kani::any();
    kani::assume(length <= LIMIT);

    initialize_prefix(length, &mut buffer);
}

fn main() {
    println!("Hello, world!");
}
