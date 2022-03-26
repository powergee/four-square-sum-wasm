use super::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_helper_functions() {
    assert_eq!(is_perfect_square(&BigInt::from(1)), true);
    assert_eq!(is_perfect_square(&BigInt::from(2)), false);
    assert_eq!(is_perfect_square(&BigInt::from(9)), true);
    assert_eq!(is_perfect_square(&BigInt::from(10)), false);
    assert_eq!(is_perfect_square(&BigInt::from(239484895234_i64).pow(2)), true);
}

#[wasm_bindgen_test]
fn test_miller_rabin() {
    assert_eq!(is_prime(&BigInt::from(1_i32)), false);
    assert_eq!(is_prime(&BigInt::from(2_i32)), true);
    assert_eq!(is_prime(&BigInt::from(3_i32)), true);
    assert_eq!(is_prime(&BigInt::from(4_i32)), false);
    assert_eq!(is_prime(&BigInt::from(11_i32)), true);
    assert_eq!(is_prime(&BigInt::from(4397_i32)), true);
    assert_eq!(is_prime(&"18446744073709551557".parse::<BigInt>().unwrap()), true);
}

#[wasm_bindgen_test]
fn test_pollard_rho() {
    assert_eq!(validate_factors(&factorize(&BigInt::from(2_i32)), &vec![BigInt::from(2_i32)]), true);
    assert_eq!(validate_factors(&factorize(&BigInt::from(3_i32)), &vec![BigInt::from(3_i32)]), true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(128_i32)),
            &vec![
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32),
                BigInt::from(2_i32)
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1804289383_i64)),
            &vec![
                BigInt::from(13_i64),
                BigInt::from(421_i64),
                BigInt::from(329671_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(424238335_i64)),
            &vec![
                BigInt::from(5_i64),
                BigInt::from(23_i64),
                BigInt::from(157_i64),
                BigInt::from(23497_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(783368690_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(5_i64),
                BigInt::from(13_i64),
                BigInt::from(67_i64),
                BigInt::from(89939_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1303455736_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(11_i64),
                BigInt::from(97_i64),
                BigInt::from(311_i64),
                BigInt::from(491_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(278722862_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(11_i64),
                BigInt::from(113_i64),
                BigInt::from(191_i64),
                BigInt::from(587_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(635723058_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(3_i64),
                BigInt::from(17_i64),
                BigInt::from(823_i64),
                BigInt::from(7573_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1131176229_i64)),
            &vec![
                BigInt::from(3_i64),
                BigInt::from(19_i64),
                BigInt::from(19845197_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(756898537_i64)),
            &vec![
                BigInt::from(127_i64),
                BigInt::from(5959831_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(149798315_i64)),
            &vec![
                BigInt::from(5_i64),
                BigInt::from(181_i64),
                BigInt::from(165523_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1424268980_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(2_i64),
                BigInt::from(5_i64),
                BigInt::from(3331_i64),
                BigInt::from(21379_i64),
            ]),
        true);

    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(1842800140108386207_i64)),
            &vec![
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(7_i64),
                BigInt::from(7_i64),
                BigInt::from(7_i64),
                BigInt::from(521_i64),
                BigInt::from(881_i64),
                BigInt::from(433517587_i64),
            ]),
        true);
    
    assert_eq!(
        validate_factors(
            &factorize(&BigInt::from(3535206131977819650_i64)),
            &vec![
                BigInt::from(2_i64),
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(3_i64),
                BigInt::from(5_i64),
                BigInt::from(5_i64),
                BigInt::from(13_i64),
                BigInt::from(97_i64),
                BigInt::from(149_i64),
                BigInt::from(12689_i64),
                BigInt::from(1098379_i64),
            ]),
        true);
}

#[wasm_bindgen_test]
fn test_four_square_random_but_fast() {
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1_i32), false)), BigInt::from(1_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2_i32), false)), BigInt::from(2_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3_i32), false)), BigInt::from(3_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(123_i32), false)), BigInt::from(123_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(123232_i32), false)), BigInt::from(123232_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1804289383_i64), false)), BigInt::from(1804289383_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(846930886_i64), false)), BigInt::from(846930886_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1681692777_i64), false)), BigInt::from(1681692777_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1714636915_i64), false)), BigInt::from(1714636915_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1957747793_i64), false)), BigInt::from(1957747793_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(424238335_i64), false)), BigInt::from(424238335_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(719885386_i64), false)), BigInt::from(719885386_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1649760492_i64), false)), BigInt::from(1649760492_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(596516649_i64), false)), BigInt::from(596516649_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1189641421_i64), false)), BigInt::from(1189641421_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1842800140108386207_i64), false)), BigInt::from(1842800140108386207_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2319442335322128615_i64), false)), BigInt::from(2319442335322128615_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2177281983706900224_i64), false)), BigInt::from(2177281983706900224_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2243420669815203258_i64), false)), BigInt::from(2243420669815203258_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3150919813662214902_i64), false)), BigInt::from(3150919813662214902_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2099491376865084255_i64), false)), BigInt::from(2099491376865084255_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2890157740621932327_i64), false)), BigInt::from(2890157740621932327_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1474775625792949608_i64), false)), BigInt::from(1474775625792949608_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3119718353897036920_i64), false)), BigInt::from(3119718353897036920_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3535206131977819650_i64), false)), BigInt::from(3535206131977819650_i64));
    assert_eq!(get_square_sum(&find_solution(&"556241405847937766541".parse::<BigInt>().unwrap(), false)), "556241405847937766541".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8035281690184899849238".parse::<BigInt>().unwrap(), false)), "8035281690184899849238".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5666108380855761692650".parse::<BigInt>().unwrap(), false)), "5666108380855761692650".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"99253663622552392578064".parse::<BigInt>().unwrap(), false)), "99253663622552392578064".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7561187792538186301056".parse::<BigInt>().unwrap(), false)), "7561187792538186301056".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"2497474809640410455936".parse::<BigInt>().unwrap(), false)), "2497474809640410455936".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7105174348442182498825".parse::<BigInt>().unwrap(), false)), "7105174348442182498825".parse::<BigInt>().unwrap());
}

#[wasm_bindgen_test]
fn test_four_square_optimal_but_slow() {
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1_i32), true)), BigInt::from(1_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2_i32), true)), BigInt::from(2_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3_i32), true)), BigInt::from(3_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(123_i32), true)), BigInt::from(123_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(123232_i32), true)), BigInt::from(123232_i32));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1804289383_i64), true)), BigInt::from(1804289383_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(846930886_i64), true)), BigInt::from(846930886_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1681692777_i64), true)), BigInt::from(1681692777_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1714636915_i64), true)), BigInt::from(1714636915_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1957747793_i64), true)), BigInt::from(1957747793_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(424238335_i64), true)), BigInt::from(424238335_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(719885386_i64), true)), BigInt::from(719885386_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1649760492_i64), true)), BigInt::from(1649760492_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(596516649_i64), true)), BigInt::from(596516649_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1189641421_i64), true)), BigInt::from(1189641421_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1842800140108386207_i64), true)), BigInt::from(1842800140108386207_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2319442335322128615_i64), true)), BigInt::from(2319442335322128615_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2177281983706900224_i64), true)), BigInt::from(2177281983706900224_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2243420669815203258_i64), true)), BigInt::from(2243420669815203258_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3150919813662214902_i64), true)), BigInt::from(3150919813662214902_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2099491376865084255_i64), true)), BigInt::from(2099491376865084255_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(2890157740621932327_i64), true)), BigInt::from(2890157740621932327_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(1474775625792949608_i64), true)), BigInt::from(1474775625792949608_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3119718353897036920_i64), true)), BigInt::from(3119718353897036920_i64));
    assert_eq!(get_square_sum(&find_solution(&BigInt::from(3535206131977819650_i64), true)), BigInt::from(3535206131977819650_i64));
    assert_eq!(get_square_sum(&find_solution(&"556241405847937766541".parse::<BigInt>().unwrap(), true)), "556241405847937766541".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"8035281690184899849238".parse::<BigInt>().unwrap(), true)), "8035281690184899849238".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"5666108380855761692650".parse::<BigInt>().unwrap(), true)), "5666108380855761692650".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"99253663622552392578064".parse::<BigInt>().unwrap(), true)), "99253663622552392578064".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7561187792538186301056".parse::<BigInt>().unwrap(), true)), "7561187792538186301056".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"2497474809640410455936".parse::<BigInt>().unwrap(), true)), "2497474809640410455936".parse::<BigInt>().unwrap());
    assert_eq!(get_square_sum(&find_solution(&"7105174348442182498825".parse::<BigInt>().unwrap(), true)), "7105174348442182498825".parse::<BigInt>().unwrap());
}

fn validate_factors(v1: &Vec<BigInt>, v2: &Vec<BigInt>) -> bool {
    if v1.len() != v2.len() {
        return false;
    } else {
        for i in 0..v1.len() {
            if v1[i] != v2[i] {
                return false;
            }
        }
        return true;
    }
}

fn get_square_sum(q: &Quadruple) -> BigInt {
    let mut sum = BigInt::from(0_i32);
    for i in 0..4 {
        sum += &q[i]*&q[i];
    }
    return sum;
}