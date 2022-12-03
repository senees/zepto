enum State {
  BeginNumber,
  LeadingZeros,
  DigitsBefore,
  DigitsAfter,
  ExponentSign,
  ExponentLeadingZeros,
  ExponentDigits,
}

///
pub fn parse(input: &str) -> (bool, u64, u64, i32) {
  let mut state = State::BeginNumber;
  let mut sign = false;
  let mut exponent = 0_i32;
  let mut exponent_base = 0_i32;
  let mut exponent_sign = false;
  let mut value = 0_u128;
  for ch in input.chars() {
    match state {
      State::BeginNumber => match ch {
        '-' => {
          sign = true;
          state = State::LeadingZeros;
        }
        '+' | '0' => state = State::LeadingZeros,
        '1'..='9' => {
          value = value * 10 + ((ch as u8) - '0' as u8) as u128;
          state = State::DigitsBefore;
        }
        _ => panic!("ERROR1"),
      },
      State::LeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          value = value * 10 + ((ch as u8) - '0' as u8) as u128;
          state = State::DigitsBefore;
        }
        '.' => state = State::DigitsAfter,
        _ => panic!("ERROR2"),
      },
      State::DigitsBefore => match ch {
        '0'..='9' => {
          value = value * 10 + ((ch as u8) - '0' as u8) as u128;
        }
        _ => panic!("ERROR3"),
      },
      State::DigitsAfter => match ch {
        '0'..='9' => {
          exponent -= 1;
          value = value * 10 + ((ch as u8) - '0' as u8) as u128;
        }
        'E' | 'e' => state = State::ExponentSign,
        _ => panic!("ERROR4"),
      },
      State::ExponentSign => match ch {
        '+' | '0' => state = State::ExponentLeadingZeros,
        '-' => {
          exponent_sign = true;
          state = State::ExponentLeadingZeros;
        }
        '1'..='9' => {
          exponent_base = exponent_base * 10 + ((ch as u8) - '0' as u8) as i32;
          state = State::ExponentDigits;
        }
        _ => panic!("ERROR5"),
      },
      State::ExponentLeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          exponent_base = exponent_base * 10 + ((ch as u8) - '0' as u8) as i32;
          state = State::ExponentDigits;
        }
        _ => panic!("ERROR6"),
      },
      State::ExponentDigits => match ch {
        '0'..='9' => {
          exponent_base = exponent_base * 10 + ((ch as u8) - '0' as u8) as i32;
        }
        _ => panic!("ERROR7"),
      },
    }
  }
  if exponent_sign {
    exponent_base = -exponent_base;
  }
  (sign, (value >> 64) as u64, value as u64, exponent + exponent_base)
}
