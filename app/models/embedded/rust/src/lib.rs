#[no_mangle]
pub extern fn nth_prime(num: u32) -> u32 {
    let mut i     = 0;
    let mut count = 0;
    while count <= num {
        i += 1;
        if prime(i) {
            count += 1;
        }
    }
    i
}

fn prime(num: u32) -> bool {
    let mut i = 2;
    while i < num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
