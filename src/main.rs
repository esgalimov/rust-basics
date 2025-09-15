
mod calculator {
    type CalcType = i32;

    #[derive(Default)]
    pub struct Calc {
        pub reg: CalcType,
    }

    impl std::fmt::Display for Calc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "reg = {}", self.reg)
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
            self.reg += rhs;
            self
        }

        fn sub(mut self, rhs: CalcType) -> Self {
            self.reg -= rhs;
            self
        }

        fn mul(mut self, rhs: CalcType) -> Self {
            self.reg *= rhs;
            self
        }

        fn div(mut self, rhs: CalcType) -> Self {
            self.reg /= rhs;
            self
        }
    }
}

use crate::calculator::Math;

fn main() {
    let calc = calculator::Calc{reg: 1}.add(2).sub(1).mul(10).div(5);

    println!("{calc}");
}
