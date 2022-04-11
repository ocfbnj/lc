struct Solution;

struct Complex {
    real: i32,
    imag: i32,
}

impl ToString for Complex {
    fn to_string(&self) -> String {
        format!("{}+{}i", self.real, self.imag)
    }
}

impl From<String> for Complex {
    fn from(s: String) -> Self {
        let arr = s.split('+').collect::<Vec<_>>();
        Complex {
            real: arr[0].parse().unwrap(),
            imag: arr[1][..arr[1].len() - 1].parse().unwrap(),
        }
    }
}

impl std::ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        let real = self.real * rhs.real - self.imag * rhs.imag;
        let imag = self.real * rhs.imag + self.imag * rhs.real;
        Complex { real, imag }
    }
}

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let num1: Complex = num1.into();
        let num2: Complex = num2.into();
        (num1 * num2).to_string()
    }
}

fn main() {
    assert_eq!(
        Solution::complex_number_multiply("1+1i".to_owned(), "1+1i".to_owned()),
        "0+2i".to_owned()
    );

    assert_eq!(
        Solution::complex_number_multiply("1+-1i".to_owned(), "1+-1i".to_owned()),
        "0+-2i".to_owned()
    );
}
