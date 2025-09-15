
mod calculator {
    type CalcType = i32;

    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        RegOverflow,
    }

    impl std::fmt::Display for Calc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.reg {
                Ok(value) =>  write!(f, "reg = {}", value),
                Err(value) => {
                    match value {
                        MathError::DivisionByZero => write!(f, "Division by zero"),
                        MathError::RegOverflow => write!(f, "Register overflow"),
                    }
                }
            }
        }
    }

    pub struct Calc {
        pub reg: Result<CalcType, MathError>,
    }

    impl Default for Calc {
        fn default() -> Self {
            Self { reg: Ok(0) }
        }
    }

    pub trait Math {
        fn add(self, rhs: CalcType) -> Self;
        fn sub(self, rhs: CalcType) -> Self;
        fn mul(self, rhs: CalcType) -> Self;
        fn div(self, rhs: CalcType) -> Self;
    }

    impl Math for Calc {
        fn add(mut self, rhs: CalcType) -> Self {
            match self.reg {
                Ok(value) => self.reg = value.checked_add(rhs).ok_or(MathError::RegOverflow),
                Err(_) => {}
            }
            self
        }

        fn sub(mut self, rhs: CalcType) -> Self {
            match self.reg {
                Ok(value) => self.reg = value.checked_sub(rhs).ok_or(MathError::RegOverflow),
                Err(_) => {}
            }
            self
        }

        fn mul(mut self, rhs: CalcType) -> Self {
            match self.reg {
                Ok(value) => self.reg = value.checked_mul(rhs).ok_or(MathError::RegOverflow),
                Err(_) => {}
            }
            self
        }

        fn div(mut self, rhs: CalcType) -> Self {
            if rhs == 0 {
                self.reg = Err(MathError::DivisionByZero);
            }
            else {
                self.reg = self.reg.map(|value| value / rhs);
            }
            self
        }
    }
}

use crate::calculator::Math;
use crate::calculator::Calc;

fn main() {
    let calc = Calc::default().add(2).sub(1).mul(10).div(1);

    println!("{calc}");
}

#[cfg(test)]
mod test {
    use crate::calculator::Math;
    use crate::calculator::Calc;

    #[test]
    fn simple_addition() {
        let res = Calc::default().add(2).add(2);
        assert_eq!("reg = 4", res.to_string());
    }

    #[test]
    fn overflow() {
        let res = Calc::default().add(i32::MAX).add(1);
        assert_eq!("Register overflow", res.to_string());
    }

    #[test]
    fn division_by_zero() {
        let res = Calc::default().div(0);
        assert_eq!("Division by zero", res.to_string());
    }
}