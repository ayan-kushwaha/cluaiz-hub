use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct MathRequest {
    expression: String,
    #[serde(default)]
    precision: Option<usize>,
}

#[derive(Serialize)]
struct MathResponse {
    status: String,
    expression: String,
    result: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut u8, len: u32) {
    if !ptr.is_null() && len > 0 {
        let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let input_bytes = if len > 0 && !ptr.is_null() {
        core::slice::from_raw_parts(ptr, len as usize)
    } else {
        &[]
    };

    let response = match serde_json::from_slice::<MathRequest>(input_bytes) {
        Ok(req) => match evaluate_expression(&req.expression) {
            Ok(val) => {
                let formatted = if let Some(p) = req.precision {
                    Some(format!("{:.prec$}", val, prec = p))
                } else {
                    Some(format!("{}", val))
                };

                MathResponse {
                    status: "success".into(),
                    expression: req.expression,
                    result: Some(val),
                    formatted,
                    error: None,
                }
            }
            Err(e) => MathResponse {
                status: "error".into(),
                expression: req.expression,
                result: None,
                formatted: None,
                error: Some(e),
            },
        },
        Err(_) => MathResponse {
            status: "error".into(),
            expression: String::new(),
            result: None,
            formatted: None,
            error: Some("Invalid input JSON payload. Expected {\"expression\": \"...\"}".into()),
        },
    };

    let output = serde_json::to_vec(&response)
        .unwrap_or_else(|_| Vec::from(b"{\"status\":\"error\",\"error\":\"Serialization failure\"}".as_slice()));

    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    core::mem::forget(output);

    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}

// ── Recursive Descent Math Evaluator ──

struct Parser<'a> {
    chars: core::iter::Peekable<core::str::Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn parse_expression(&mut self) -> Result<f64, String> {
        let mut left = self.parse_term()?;
        self.skip_whitespace();
        while let Some(&op) = self.chars.peek() {
            if op == '+' || op == '-' {
                self.chars.next();
                let right = self.parse_term()?;
                if op == '+' {
                    left += right;
                } else {
                    left -= right;
                }
                self.skip_whitespace();
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<f64, String> {
        let mut left = self.parse_power()?;
        self.skip_whitespace();
        while let Some(&op) = self.chars.peek() {
            if op == '*' || op == '/' || op == '%' {
                self.chars.next();
                let right = self.parse_power()?;
                match op {
                    '*' => left *= right,
                    '/' => {
                        if right == 0.0 {
                            return Err("Division by zero".into());
                        }
                        left /= right;
                    }
                    '%' => {
                        if right == 0.0 {
                            return Err("Modulo by zero".into());
                        }
                        left %= right;
                    }
                    _ => unreachable!(),
                }
                self.skip_whitespace();
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<f64, String> {
        let left = self.parse_factor()?;
        self.skip_whitespace();
        if let Some(&'^') = self.chars.peek() {
            self.chars.next();
            let right = self.parse_power()?; // Right-associative exponentiation
            return Ok(pow_f64(left, right));
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        self.skip_whitespace();
        match self.chars.peek() {
            Some(&'+') => {
                self.chars.next();
                self.parse_factor()
            }
            Some(&'-') => {
                self.chars.next();
                let val = self.parse_factor()?;
                Ok(-val)
            }
            Some(&'(') => {
                self.chars.next();
                let val = self.parse_expression()?;
                self.skip_whitespace();
                if self.chars.next() == Some(')') {
                    Ok(val)
                } else {
                    Err("Mismatched opening parenthesis".into())
                }
            }
            Some(&c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(&c) if c.is_alphabetic() => self.parse_identifier_or_function(),
            Some(&c) => Err(format!("Unexpected character '{}'", c)),
            None => Err("Unexpected end of expression".into()),
        }
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let mut s = String::new();
        let mut has_dot = false;
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.chars.next();
            } else if c == '.' && !has_dot {
                has_dot = true;
                s.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        s.parse::<f64>()
            .map_err(|_| format!("Invalid number token '{}'", s))
    }

    fn parse_identifier_or_function(&mut self) -> Result<f64, String> {
        let mut name = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                name.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        self.skip_whitespace();
        if let Some(&'(') = self.chars.peek() {
            // Function call
            self.chars.next();
            let arg = self.parse_expression()?;
            self.skip_whitespace();
            if self.chars.next() != Some(')') {
                return Err(format!("Expected closing ')' after function argument in {}", name));
            }

            match name.as_str() {
                "abs" => Ok(if arg < 0.0 { -arg } else { arg }),
                "sqrt" => {
                    if arg < 0.0 {
                        Err("Square root of negative number".into())
                    } else {
                        Ok(sqrt_f64(arg))
                    }
                }
                "floor" => Ok(floor_f64(arg)),
                "ceil" => Ok(ceil_f64(arg)),
                "round" => Ok(round_f64(arg)),
                "sin" => Ok(sin_f64(arg)),
                "cos" => Ok(cos_f64(arg)),
                "tan" => {
                    let c = cos_f64(arg);
                    if c == 0.0 {
                        Err("Tangent undefined (division by zero)".into())
                    } else {
                        Ok(sin_f64(arg) / c)
                    }
                }
                "ln" | "log" => {
                    if arg <= 0.0 {
                        Err("Logarithm of non-positive number".into())
                    } else {
                        Ok(ln_f64(arg))
                    }
                }
                "exp" => Ok(exp_f64(arg)),
                _ => Err(format!("Unknown math function '{}'", name)),
            }
        } else {
            // Constants
            match name.as_str() {
                "pi" | "PI" => Ok(core::f64::consts::PI),
                "e" | "E" => Ok(core::f64::consts::E),
                _ => Err(format!("Unknown constant or variable '{}'", name)),
            }
        }
    }
}

pub fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let mut parser = Parser::new(expr);
    let result = parser.parse_expression()?;
    parser.skip_whitespace();
    if parser.chars.peek().is_some() {
        return Err("Unexpected trailing characters in expression".into());
    }
    Ok(result)
}

// ── no_std Math Approximations (Pure deterministic float arithmetic) ──

fn sqrt_f64(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    let mut guess = x / 2.0;
    for _ in 0..20 {
        guess = (guess + x / guess) / 2.0;
    }
    guess
}

fn floor_f64(x: f64) -> f64 {
    let i = x as i64;
    if x < (i as f64) {
        (i - 1) as f64
    } else {
        i as f64
    }
}

fn ceil_f64(x: f64) -> f64 {
    let i = x as i64;
    if x > (i as f64) {
        (i + 1) as f64
    } else {
        i as f64
    }
}

fn round_f64(x: f64) -> f64 {
    floor_f64(x + 0.5)
}

fn pow_f64(base: f64, exp: f64) -> f64 {
    if exp == 0.0 {
        return 1.0;
    }
    if exp == 1.0 {
        return base;
    }
    let exp_int = exp as i64;
    if exp == exp_int as f64 {
        if exp_int > 0 {
            let mut res = 1.0;
            let mut b = base;
            let mut e = exp_int;
            while e > 0 {
                if e % 2 == 1 {
                    res *= b;
                }
                b *= b;
                e /= 2;
            }
            return res;
        } else {
            return 1.0 / pow_f64(base, -exp);
        }
    }
    // exp(exp * ln(base))
    if base <= 0.0 {
        0.0
    } else {
        exp_f64(exp * ln_f64(base))
    }
}

fn ln_f64(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    // Halley's method for ln(x)
    let mut y = (x - 1.0) / (x + 1.0);
    let y2 = y * y;
    let mut term = y;
    let mut sum = y;
    for k in 1..25 {
        term *= y2;
        sum += term / (2 * k + 1) as f64;
    }
    2.0 * sum
}

fn exp_f64(x: f64) -> f64 {
    let mut sum = 1.0;
    let mut term = 1.0;
    for i in 1..30 {
        term *= x / (i as f64);
        sum += term;
    }
    sum
}

fn sin_f64(mut x: f64) -> f64 {
    // Normalize to [-PI, PI]
    let two_pi = 2.0 * core::f64::consts::PI;
    x = x % two_pi;
    if x > core::f64::consts::PI {
        x -= two_pi;
    } else if x < -core::f64::consts::PI {
        x += two_pi;
    }

    let mut sum = x;
    let mut term = x;
    let x2 = x * x;
    for i in 1..10 {
        term *= -x2 / ((2 * i) * (2 * i + 1)) as f64;
        sum += term;
    }
    sum
}

fn cos_f64(x: f64) -> f64 {
    sin_f64(x + core::f64::consts::FRAC_PI_2)
}
