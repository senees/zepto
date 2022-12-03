use crate::parse;

fn eq(input: &str, sign: bool, w1: u64, w0: u64, exp: i32) {
  let (actual_sign, actual_w1, actual_w0, actual_exponent) = parse(input);
  assert_eq!(sign, actual_sign);
  assert_eq!(w1, actual_w1);
  assert_eq!(w0, actual_w0);
  assert_eq!(exp, actual_exponent);
}

#[test]
fn _0001() {
  eq("12", false, 0, 12, 0);
}

#[test]
fn _0002() {
  eq("+12", false, 0, 12, 0);
}

#[test]
fn _0003() {
  eq("-12", true, 0, 12, 0);
}

#[test]
fn _0004() {
  eq("000001", false, 0, 1, 0);
}

#[test]
fn _0005() {
  eq("+000001", false, 0, 1, 0);
}

#[test]
fn _0006() {
  eq("-000001", true, 0, 1, 0);
}

#[test]
fn _0007() {
  eq("0.3", false, 0, 3, -1);
}

#[test]
fn _0008() {
  eq("0.3E2", false, 0, 3, 1);
}

#[test]
fn _0009() {
  eq("0.3e2", false, 0, 3, 1);
}

#[test]
fn _0010() {
  eq("0.3E02", false, 0, 3, 1);
}

#[test]
fn _0011() {
  eq("0.3E+02", false, 0, 3, 1);
}

#[test]
fn _0012() {
  eq("0.3E-02", false, 0, 3, -3);
}

#[test]
fn _0013() {
  eq("0.00003E-02", false, 0, 3, -7);
}

#[test]
fn _0014() {
  eq("9999999999999999999999999999999999", false, 0x1ed09bead87c0, 0x378d8e63ffffffff, 0);
}

#[test]
fn _0015() {
  eq(
    "99999999999999999999999999999999999999999999999999999999999999999",
    false,
    0x1ed09bead87c0,
    0x378d8e63ffffffff,
    0,
  );
}
