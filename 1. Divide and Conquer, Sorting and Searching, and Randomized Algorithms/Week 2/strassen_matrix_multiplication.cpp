/**
 * Strassens' matrix multiplication.
 *
 * Divide matrix into 4 equal quadrants. Use Strassen's addition and subraction trick to compute
 * only 7 recursively calls to find the actual product.
 *
 * Currently works for square matrix of same size only.
**/

#include <iostream>

// Matrix of size [n]
class Matrix {
public:
    int **matrix;
    size_t n;

    // Allocate size for an N x N matrix.
    Matrix(size_t n);

    // Multiply matrices using Strassen's multiplication.
    static Matrix strassen_multiply(Matrix &first, Matrix &second);

    // Multiply matrices using traditional method.
    // Formula: Matrix[i][j] = Sigma(k = 1, n) { Matrix[i][k] * Matrix[k][j] }
    static Matrix traditional_multiply(Matrix &first, Matrix &second);

    // Returns the sum of two matrices.
    Matrix operator + (Matrix &other);

    // Return the difference of two matrices.
    Matrix operator - (Matrix &other);

    // Housekeeping.
    ~Matrix(void);
};


Matrix Matrix::strassen_multiply(Matrix &first, Matrix &second) {
    // Do traditional multiplication if size is odd or matrices are small
    if (first.n % 2 == 1 || second.n % 2 == 1 || first.n < 50 || second.n < 50) {
        return traditional_multiply(first, second);
    }

    // Divide both matrices into equal quadrants. (a b c d for first, and e f g h for second)
    size_t size = first.n / 2;
    Matrix a(size), b(size), c(size), d(size);
    Matrix e(size), f(size), g(size), h(size);

    // Copy over the elements into these quadrants.
    for (int row = 0; row < size; row++) {
        for (int col = 0; col < size; col++) {
            // First matrix division
            a.matrix[row][col] = first.matrix[row][col];
            b.matrix[row][col] = first.matrix[row][size + col];
            c.matrix[row][col] = first.matrix[size + row][col];
            d.matrix[row][col] = first.matrix[size + row][size + col];

            // Second matrix division.
            e.matrix[row][col] = second.matrix[row][col];
            f.matrix[row][col] = second.matrix[row][size + col];
            g.matrix[row][col] = second.matrix[size + row][col];
            h.matrix[row][col] = second.matrix[size + row][size + col];
        }
    }

    // Compute Strassen's products.
    Matrix temp = f - h;
    Matrix product1 = strassen_multiply(a, temp);

    Matrix temp2 = a + b;
    Matrix product2 = strassen_multiply(temp2, h);

    Matrix temp3 = c + d;
    Matrix product3 = strassen_multiply(temp3, e);

    Matrix temp4 = g - e;
    Matrix product4 = strassen_multiply(d, temp4);

    Matrix temp5 = a + d, temp6 = e + h;
    Matrix product5 = strassen_multiply(temp5, temp6);

    Matrix temp7 = b - d, temp8 = g + h;
    Matrix product6 = strassen_multiply(temp7, temp8);

    Matrix temp9 = a - c, temp10 = e + f;
    Matrix product7 = strassen_multiply(temp9, temp10);

    Matrix *result = new Matrix(first.n);

    // Re combine the above products into quadrants again.
    Matrix quad1 = product5 + product4 - product2 + product6;
    Matrix quad2 = product1 + product2;
    Matrix quad3 = product3 + product4;
    Matrix quad4 = product1 + product5 - product3 - product7;

    // Rebuild the product matrix by combining quadrants.
    for (int row = 0; row < size; row++) {
        for (int col = 0; col < size; col++) {
            result->matrix[row][col] = quad1.matrix[row][col];
            result->matrix[row][col + size] = quad2.matrix[row][col];
            result->matrix[row + size][col] = quad3.matrix[row][col];
            result->matrix[row + size][col + size] = quad4.matrix[row][col];
        }
    }

    return *result;
}

// Size of both matrices is same.
Matrix Matrix::traditional_multiply(Matrix &first, Matrix &second) {
    Matrix result(first.n);

    for (int row = 0; row < first.n; row++) {
        for (int col = 0; col < first.n; col++) {
            int sum = 0;
            for (int k = 0; k < first.n; k++) {
                sum += (first.matrix[row][k] * second.matrix[k][col]);
            }
            result.matrix[row][col] = sum;
        }
    }
    return result;
}

// Addition
Matrix Matrix::operator+(Matrix &other) {
    Matrix result(this->n);
    for (int row = 0; row < this->n; row++) {
        for (int col = 0; col < this->n; col++) {
            result.matrix[row][col] = this->matrix[row][col] + other.matrix[row][col];
        }
    }
    return result;
}

// Subtraction
Matrix Matrix::operator-(Matrix &other) {
    Matrix result(this->n);
    for (int row = 0; row < this->n; row++) {
        for (int col = 0; col < this->n; col++) {
            result.matrix[row][col] = this->matrix[row][col] - other.matrix[row][col];
        }
    }
    return result;
}

// Builds a new matrix.
Matrix::Matrix(size_t n) {
    this->n = n;
    this->matrix = (int **)malloc(n * sizeof(int *));
    for (int row = 0; row < n; row++) {
        this->matrix[row] = (int *)calloc(n, sizeof(int));
    }
}

// Free up memory.
Matrix::~Matrix(void) {
    if (this->matrix != NULL) {
        for (int row = 0; row < this->n; row++) {
            free(this->matrix[row]);
        }
        free(this->matrix);
    }
}

int main(void) {
    // Size of N x N matrix.
    const size_t N = 1024;

    srand(time(0));
    Matrix one(N), two(N);

    // Fill matrices with random numbers.
    for (size_t row = 0; row < N; row++) {
        for (size_t col = 0; col < N; col++) {
            one.matrix[row][col] = rand() % 100 + 1;
            two.matrix[row][col] = rand() % 100 + 1;
        }
    }

    // // Compute Strassen's product.
    Matrix strassen_product = Matrix::strassen_multiply(one, two);
    std::cout << "Strassen's product done!\n";

    // Compute product using traditional method.
    Matrix traditional_product = Matrix::traditional_multiply(one, two);
    std::cout << "Traditional product done!\n";

    // Test whether both products are the same.
    for (size_t row = 0; row < N; row++) {
        for (size_t col = 0; col < N; col++) {
            assert(strassen_product.matrix[row][col] == traditional_product.matrix[row][col]);
        }
    }

    // We are here, means all good!.
    std::cout << "Product verified!\n";
}