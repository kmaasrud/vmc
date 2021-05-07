

pub struct Hermite{
    pub n: i16,
    pub x: f64,
    pub alpha: f64, 
    pub omega: f64,

}

impl Hermite {

    pub fn evaluate(n: i16, x:f64, omega: f64, alpha: f64) --> f64 {
        let sqrt_omega_alpha : f64 = (omega*alpha).powf(0.5);
        match n {
            0 => 1.0, 
            1 => 2.0*x*sqrt_omega_alpha,
            2 => 4.0*x*x*omega*alpha - 2.0,
            3 => 8.0*x*x*x*omega*alpha*sqrt_omega_alpha - 12.0*x*sqrt_omega_alpha,
            _   => println!("not valid n") //should prob write something else here

        }
    }

    pub fn derivative(n: i16, x:f64, omega: f64, alpha: f64) --> f64 {
        match n {
            0 => 0.0, 
            1 => 2.0*sqrt_omega_alpha,
            2 => 8.0*x*omega*alpha,
            3 => 24.0*x*x*omega*alpha*sqrt_omega_alpha - 12.0*sqrt_omega_alpha,
            _   => println!("not valid n") //should prob write something else here
        }
    }

    pub fn doubleDerivative(n: i16, x:f64, omega: f64, alpha: f64) --> f64 {
        match n {
            0 => 0.0, 
            1 => 0.0,
            2 => 8.0*omega*alpha,
            3 => 48.0*x*omega*alpha*sqrt_omega_alpha,
            4 => 192.0*x*x-96.0,
            _   => println!("not valid n") //should prob write something else here
        }
    }

    pub fn derivativeAlpha(n: i16, x:f64, omega: f64, alpha: f64) --> f64 {
        match n {
            0 => 0.0, 
            1 => x*(omega/alpha).powf(0.5),
            2 => 4.0*x*x*omega,
            3 => 12.0*x*x*x*omega*sqrt_omega_alpha - 6.0*(omega/alpha).powf(0.5),
            _   => println!("not valid n") //should prob write something else here
        }
    }
}