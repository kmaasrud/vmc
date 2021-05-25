use crate::Vector::{self, *};

/// Struct that represents a single particle.
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vector,
    pub dim: usize,
    pub qforce: Vector,
    pub energy_state: Vector,
}

impl Particle {
    /// Creates a new particle with a given dimensionality.
    /// The particle's initial position is set to 0.
    pub fn new(dim: usize) -> Result<Self, String> {
        let (position, energy_state) = match dim {
            1 => (D1(0.), D1(0.)),
            2 => (D2(0., 0.), D2(0., 0.)),
            3 => (D3(0., 0., 0.), D3(0., 0., 0.)),
            _ => return Err("Unsupported dimensionality.".to_owned()),
        };

        Ok(Particle{ position, qforce: position, dim, energy_state })
    }

    pub fn from_vector(position: Vector) -> Self {
        let (dim, qforce, energy_state) = match position {
            D1(_) => (1, D1(0.), D1(0.)),
            D2(_,_) => (2, D2(0., 0.), D2(0., 0.)),
            D3(_,_,_) => (3, D3(0., 0., 0.), D3(0., 0., 0.)),
        };
        Particle { position, qforce, dim, energy_state }
    }

    /// Computes the squared sum of each coordinate.
    pub fn squared_sum(&self) -> f64 {
        match self.position {
            D1(x) => x.powi(2),
            D2(x, y) => x.powi(2) + y.powi(2),
            D3(x, y, z) => x.powi(2) + y.powi(2) + z.powi(2),
        }
    }

    /// Computes the squared sum of each coordinate, but the z-component is scaled by a factor
    pub fn squared_sum_scaled_z(&self, factor: f64) -> f64 {
        match self.position {
            D1(_) | D2(_,_) => self.squared_sum(),
            D3(x, y, z) => x.powi(2) + y.powi(2) + factor * z.powi(2),
        }
    }

    /// Returns the distance from this particle to other
    pub fn distance_to(&self, other: &Particle) -> Result<f64, String> {
        match (self.position, other.position) {
            (D1(x1), D1(x2)) => Ok((x1 - x2).powi(2).sqrt()),
            (D2(x1, y1), D2(x2, y2)) => Ok(((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()),
            (D3(x1, y1, z1), D3(x2, y2, z2)) => Ok(((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()),
            _ => Err("Dimensions do not match".to_owned()),
        }
    }

    // Ugh... This is pretty darn ugly, but I can't think of a better way to do it with enums...
    // At least this should be loads faster than our previous approach, and it is error-safe.
    /// Adds 'bump_size' to the component specified by 'dim'
    pub fn bump_at_dim(&mut self, dim: usize, bump_size: f64) {
        self.position = match dim {
            0 => match self.position {
                D1(x) => D1(x + bump_size),
                D2(x, y) => D2(x + bump_size, y),
                D3(x, y, z) => D3(x + bump_size, y, z),
            }
            1 => match self.position {
                D1(_) => self.position,
                D2(x, y) => D2(x, y + bump_size),
                D3(x, y, z) => D3(x, y + bump_size, z),
            }
            2 => match self.position {
                D1(_) | D2(_, _) => self.position,
                D3(x, y, z) => D3(x, y, z + bump_size),
            }
            _ => self.position
        };
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squared_sum_scaled_z() {
        let tol: f64 = 1e-12;
        let want: f64 = 1402.624001;
        let particle = Particle::from_vector(D3(2.5, 1.001, 68.2));
        let got: f64 = particle.squared_sum_scaled_z(0.3);
        assert!((want - got).abs() < tol);
    }

    #[test]
    fn test_distance_to() {
        let tol: f64 = 0.00001;
        let want: f64 = 6.25744;
        let got: f64 = Particle::from_vector(D3(3., 5.666, 8.))
            .distance_to(&Particle::from_vector(D3(2., 9., 13.2))).unwrap();
        assert!((want - got).abs() < tol);
    }

    #[test]
    fn test_bump_at_dim() {
        let want = D3(1., 1., 1.6);
        let mut got = Particle::from_vector(D3(1., 1., 1.));
        got.bump_at_dim(2, 0.6);
        assert!(matches!(got.position, want));
    }
}
