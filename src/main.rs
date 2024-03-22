mod Matrix;



fn main() {
    let mut matrix = Matrix::Matrix::new(3, 3);

    
    matrix.random();
    matrix.print("Matrix".to_string());
    


    
}