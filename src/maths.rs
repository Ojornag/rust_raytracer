use std::ops;

// Floating point equal
const EPSILON: f64 = 0.0001;

pub fn equal(a: f64, b: f64) -> bool {
    return (a - b).abs() < EPSILON;
}

// Vectors
#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

// Print vectors
impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "┌────{}────┐\n", "────┬────".repeat(3)).unwrap();
        write!(f, "│{:^8.2}│{:^8.2}│{:^8.2}│{:^8.2}│\n", self.x, self.y, self.z, self.w).unwrap();
        write!(f, "└────{}────┘\n", "────┴────".repeat(3)).unwrap();

        return Ok(());
    }
}

// Overload operators with the corresponding vector operations
//
// Addition
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        return Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        };
    }
}

// Subtraction
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        return Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        };
    }
}

// Negation
impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        return Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        };
    }
}

// Scalar multiplication
impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        return Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        };
    }
}

// Dot product
impl ops::Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, rhs: Vector) -> f64 {
        return
            self.x * rhs.x +
            self.y * rhs.y +
            self.z * rhs.z +
            self.w * rhs.w;
    }
}

// Cross product
impl ops::BitXor<Vector> for Vector {
    type Output = Vector;

    fn bitxor(self, rhs: Vector) -> Vector {
        return Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: 0.0
        };
    }
}

// Add all other functions to vectors
impl Vector {
    pub fn new(vector: Vec<f64>) -> Vector {
        let mut final_vec = vec![0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            if i < vector.len() {
                final_vec[i] = vector[i];
                continue;
            }
            final_vec[i] = 0.0;
        }

        return Vector {
            x: final_vec[0],
            y: final_vec[1],
            z: final_vec[2],
            w: final_vec[3]
        };
    }

    pub fn magnitude(&self) -> f64 {
        return (
            self.x * self.x +
            self.y * self.y +
            self.z * self.z +
            self.w * self.w
        ).sqrt();
    }

    pub fn normalized(&self) -> Vector {
        let magnitude = self.magnitude();

        return Vector {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        };
    }
}

// Matrices
pub struct Matrix {
    pub matrix: Vec<Vec<f64>>,
    size: usize
}

// Print matrices
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "┌────{}────┐\n", "────┬────".repeat(self.size - 1)).unwrap();
        for y in 0..self.size {
            for x in 0..self.size {
                write!(f, "│{:^8.2}", self.matrix[y][x]).unwrap();
            }
            write!(f, "│\n").unwrap();
            if y != self.size - 1 {
                write!(f, "├────{}────┤\n", "────┼────".repeat(self.size - 1)).unwrap();
            }
        }
        write!(f, "└────{}────┘", "────┴────".repeat(self.size - 1)).unwrap();
        return Ok(());
    }
}

// Overload operators with the corresponding matrix operations
// Matrix multiplication
impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        if self.size != rhs.size {
            panic!("Tried to multiply two matrices of different sizes");
        }

        let size = self.size;

        let mut matrix = vec![vec![0.0; size]; size];

        for y in 0..size {
            for x in 0..size {
                let mut sum: f64 = 0.0;
                for i in 0..size{
                    sum += self.matrix[y][i] * rhs.matrix[i][x];
                }
                matrix[y][x] = sum;
            }
        }

        return Matrix { matrix, size };
    }
}

// Vector matrix multiplication
impl ops::Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        if self.size != 4 {
            panic!("Tried to multiply matrix and vector with different sizes");
        }

        let vec = vec![rhs.x, rhs.y, rhs.z, rhs.w];
        let mut final_vec = vec![0.0, 0.0, 0.0, 0.0];

        for y in 0..4 {
            let mut sum: f64 = 0.0;
            for i in 0..4 {
                sum += self.matrix[y][i] * vec[i];
            }
            final_vec[y] = sum;
        }

        return Vector {
            x: final_vec[0],
            y: final_vec[1],
            z: final_vec[2],
            w: final_vec[3]
        };
    }
}

// Scalar multiplication
impl ops::Mul<f64> for Matrix{
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Matrix {
        let mut matrix = self.matrix;

        for y in 0..self.size {
            for x in 0..self.size {
                matrix[y][x] *= rhs;
            }
        }

        return Matrix { matrix, size: self.size };
    }
}

// Matrix functions
impl Matrix {
    pub fn new(matrix: Vec<Vec<f64>>) -> Matrix{
        let size = matrix.len();
        if size != matrix[0].len(){
            panic!("Matrix is not square");
        }
        return Matrix { matrix, size };
    }

    pub fn identity(size: usize) -> Matrix {
        let mut matrix = vec![vec![0.0; size]; size];

        for i in 0..size {
            matrix[i][i] = 1.0;
        }

        return Matrix { matrix, size };
    }

    pub fn transposed(&self) -> Matrix{
        let mut matrix = vec![vec![0.0; self.size]; self.size];

        for y in 0..self.size {
            for x in 0..self.size{
                matrix[x][y] = self.matrix[y][x];
            }
        }

        return Matrix { matrix, size: self.size };
    }

    pub fn determinant(&self) -> f64 {
        if self.size == 1 {
            return self.matrix[0][0];
        }
        if self.size == 2 {
            return self.matrix[0][0] * self.matrix[1][1] - self.matrix[0][1] * self.matrix[1][0];
        }
        let mut determinant: f64 = 0.0;
        for x in 0..self.size {
            determinant += self.matrix[0][x] * self.cofactor(0, x);
        }

        return determinant;
    }

    pub fn submatrix(&self, y: usize, x: usize) -> Matrix {
        let mut matrix = vec![vec![0.0; self.size - 1]; self.size - 1];

        for i in 0..self.size {
            if i == y {
                continue;
            }
            for j in 0..self.size {
                if j == x {
                    continue;
                }
                matrix[i - (i > y) as usize][j - (j > x) as usize] = self.matrix[i][j];
            }
        }

        return Matrix { matrix, size: self.size - 1 };
    }

    pub fn cofactor(&self, y: usize, x: usize) -> f64 {
        let minor = self.submatrix(y, x).determinant();
        let sign = 1 - 2 * ((x + y) as i8 % 2);

        return sign as f64 * minor;
    }

    pub fn inverse(&self) -> Matrix {
        let determinant = self.determinant();

        if determinant == 0.0 {
            panic!("Tried to invert matrix with determinant 0");
        }

        let mut matrix = vec![vec![0.0; self.size]; self.size];

        for y in 0..self.size {
            for x in 0..self.size {
                matrix[y][x] = self.cofactor(x, y) / determinant;
            }
        }

        return Matrix { matrix, size: self.size };
    }
}
