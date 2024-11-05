pub fn baby_eve(alice_broadcasts: u64, bob_broadcasts: u64, public_base: u64) -> [u64; 3] {
    // Purpose:     Crack baby DH
    // Parameters:  alice's broadcast, bob's broadcast, and the public base
    // User-input:  None
    // Prints:      Nothing
    // Returns:     Should return an array of 3 unsigned ints:
    //              Alice's secret, Bob's secret, shared secret
    // Modifies:    Nothing
    // Calls:       ?
    // Tests:       unit_test_babydh.rs
    // Status:      Done, correct, but is it ideal?
    let mut alice_sec = 0;
    let mut bob_sec = 0;

    for i in 1.. {
        if public_base.pow(i as u32) == alice_broadcasts {
            alice_sec = i;
            break;
        }
    }

    for i in 1.. {
        if public_base.pow(i as u32) == bob_broadcasts {
            bob_sec = i;
            break;
        }
    }

    let shared_secret = bob_broadcasts.pow(alice_sec as u32);

    [alice_sec, bob_sec, shared_secret]
}

pub fn big_eve(
    alice_broadcasts: u64,
    bob_broadcasts: u64,
    public_base: u64,
    public_modulus: u64,
) -> [u64; 3] {
    // Purpose:      Crack real DH (albeit not with huge numbers)
    // Parameters:   Alice's broadcast, Bob's broadcast, the public base and modulus of DH.
    // User-input:   None
    // Prints:       Nothing
    // Returns:      Should return an array of 3 ints:
    //               Alice's secret, Bob's secret, shared secret
    // Modifies:     Nothing
    // Calls:        ?
    // Test:         ./unit_tests/unit_test_babydh.rs
    // Status:       TODO delete the 0 placeholders, and replace with real computations
    let mut alice_sec = 0;
    let mut bob_sec = 0;

    for i in 1.. {
        if public_base.pow(i as u32).rem_euclid(public_modulus) == alice_broadcasts {
            alice_sec = i;
            break;
        }
    }

    for i in 1.. {
        if public_base.pow(i as u32).rem_euclid(public_modulus) == bob_broadcasts {
            bob_sec = i;
            break;
        }
    }

    let shared_secret = bob_broadcasts
        .pow(alice_sec as u32)
        .rem_euclid(public_modulus);

    [alice_sec, bob_sec, shared_secret]
}
