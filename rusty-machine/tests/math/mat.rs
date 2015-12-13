use rm::math::linalg::matrix::Matrix;
use rm::math::linalg::vector::Vector;
use libnum::abs;

#[test]
fn create_mat_new() {
	let a = Matrix::new(4, 3, vec![0.0; 12]);

	assert_eq!(a.rows, 4);
	assert_eq!(a.cols, 3);
}

#[test]
fn create_mat_zeros() {
	let a = Matrix::<f32>::zeros(10, 10);

	assert_eq!(a.rows, 10);
	assert_eq!(a.cols, 10);

	for i in 0..10
	{
		for j in 0..10
		{
			assert_eq!(a[[i,j]], 0.0);
		}
	}
}

#[test]
fn create_mat_identity() {
	let a = Matrix::<f32>::identity(4);

	assert_eq!(a.rows, 4);
	assert_eq!(a.cols, 4);

	assert_eq!(a[[0,0]], 1.0);
	assert_eq!(a[[1,1]], 1.0);
	assert_eq!(a[[2,2]], 1.0);
	assert_eq!(a[[3,3]], 1.0);

	assert_eq!(a[[0,1]], 0.0);
	assert_eq!(a[[2,1]], 0.0);
	assert_eq!(a[[3,0]], 0.0);
}

#[test]
fn create_mat_diag() {
	let a = Matrix::from_diag(&[1.0,2.0,3.0,4.0]);

	assert_eq!(a.rows, 4);
	assert_eq!(a.cols, 4);

	assert_eq!(a[[0,0]], 1.0);
	assert_eq!(a[[1,1]], 2.0);
	assert_eq!(a[[2,2]], 3.0);
	assert_eq!(a[[3,3]], 4.0);

	assert_eq!(a[[0,1]], 0.0);
	assert_eq!(a[[2,1]], 0.0);
	assert_eq!(a[[3,0]], 0.0);
}

#[test]
fn transpose_mat() {
	let a = Matrix::new(5, 2, vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]);

	let c = a.transpose();

	assert_eq!(c.cols, a.rows);
	assert_eq!(c.rows, a.cols);

	assert_eq!(a[[0,0]], c[[0,0]]);
	assert_eq!(a[[1,0]], c[[0,1]]);
	assert_eq!(a[[2,0]], c[[0,2]]);
	assert_eq!(a[[3,0]], c[[0,3]]);
	assert_eq!(a[[4,0]], c[[0,4]]);
	assert_eq!(a[[0,1]], c[[1,0]]);
	assert_eq!(a[[1,1]], c[[1,1]]);
	assert_eq!(a[[2,1]], c[[1,2]]);
	assert_eq!(a[[3,1]], c[[1,3]]);
	assert_eq!(a[[4,1]], c[[1,4]]);

}

#[test]
fn indexing_mat() {
	let a = Matrix::new(3, 2, vec![1.,2.,3.,4.,5.,6.]);

	assert_eq!(a[[0,0]], 1.0);
	assert_eq!(a[[0,1]], 2.0);
	assert_eq!(a[[1,0]], 3.0);
	assert_eq!(a[[1,1]], 4.0);
	assert_eq!(a[[2,0]], 5.0);
	assert_eq!(a[[2,1]], 6.0);
}

#[test]
fn matrix_mul() {
	let a = Matrix::new(3, 2, vec![1.,2.,3.,4.,5.,6.]);
	let b = Matrix::new(2, 3, vec![1.,2.,3.,4.,5.,6.]);

	let c = a * b;

	assert_eq!(c.rows, 3);
	assert_eq!(c.cols, 3);

	assert_eq!(c[[0,0]], 9.0);
	assert_eq!(c[[0,1]], 12.0);
	assert_eq!(c[[0,2]], 15.0);
	assert_eq!(c[[1,0]], 19.0);
	assert_eq!(c[[1,1]], 26.0);
	assert_eq!(c[[1,2]], 33.0);
	assert_eq!(c[[2,0]], 29.0);
	assert_eq!(c[[2,1]], 40.0);
	assert_eq!(c[[2,2]], 51.0);
}

#[test]
fn matrix_vec_mul() {
	let a = Matrix::new(3, 2, vec![1.,2.,3.,4.,5.,6.]);
	let b = Vector::new(vec![4.,7.]);

	let c = a * b;

	assert_eq!(c.size, 3);

	assert_eq!(c[0], 18.0);
	assert_eq!(c[1], 40.0);
	assert_eq!(c[2], 62.0);
}

#[test]
fn matrix_f32_mul() {
	let a = Matrix::new(3, 2, vec![1.,2.,3.,4.,5.,6.]);

	let c = a * 2.0;

	assert_eq!(c[[0,0]], 2.0);
	assert_eq!(c[[0,1]], 4.0);
	assert_eq!(c[[1,0]], 6.0);
	assert_eq!(c[[1,1]], 8.0);
	assert_eq!(c[[2,0]], 10.0);
	assert_eq!(c[[2,1]], 12.0);
}

#[test]
fn matrix_add() {
	let a = Matrix::new(3, 2, vec![1.,2.,3.,4.,5.,6.]);
	let b = Matrix::new(3, 2, vec![2.,3.,4.,5.,6.,7.]);

	let c = a + b;

	assert_eq!(c[[0,0]], 3.0);
	assert_eq!(c[[0,1]], 5.0);
	assert_eq!(c[[1,0]], 7.0);
	assert_eq!(c[[1,1]], 9.0);
	assert_eq!(c[[2,0]], 11.0);
	assert_eq!(c[[2,1]], 13.0);
}

#[test]
fn matrix_f32_add() {
	let a = Matrix::new(3, 2, vec![1.,2.,3.,4.,5.,6.]);
	let b = 3.0;

	let c = a + b;

	assert_eq!(c[[0,0]], 4.0);
	assert_eq!(c[[0,1]], 5.0);
	assert_eq!(c[[1,0]], 6.0);
	assert_eq!(c[[1,1]], 7.0);
	assert_eq!(c[[2,0]], 8.0);
	assert_eq!(c[[2,1]], 9.0);
}

#[test]
fn matrix_sub() {
	let a = Matrix::new(3, 2, vec![1.,2.,3.,4.,5.,6.]);
	let b = Matrix::new(3, 2, vec![2.,3.,4.,5.,6.,7.]);

	let c = a - b;

	assert_eq!(c[[0,0]], -1.0);
	assert_eq!(c[[0,1]], -1.0);
	assert_eq!(c[[1,0]], -1.0);
	assert_eq!(c[[1,1]], -1.0);
	assert_eq!(c[[2,0]], -1.0);
	assert_eq!(c[[2,1]], -1.0);
}

#[test]
fn matrix_f32_sub() {
	let a = Matrix::new(3, 2, vec![1.,2.,3.,4.,5.,6.]);
	let b = 3.0;

	let c = a - b;

	assert_eq!(c[[0,0]], -2.0);
	assert_eq!(c[[0,1]], -1.0);
	assert_eq!(c[[1,0]], 0.0);
	assert_eq!(c[[1,1]], 1.0);
	assert_eq!(c[[2,0]], 2.0);
	assert_eq!(c[[2,1]], 3.0);
}

#[test]
fn matrix_f32_div() {
	let a = Matrix::new(3, 2, vec![1.,2.,3.,4.,5.,6.]);
	let b = 3.0;

	let c = a / b;

	assert_eq!(c[[0,0]], 1.0/3.0);
	assert_eq!(c[[0,1]], 2.0/3.0);
	assert_eq!(c[[1,0]], 1.0);
	assert_eq!(c[[1,1]], 4.0/3.0);
	assert_eq!(c[[2,0]], 5.0/3.0);
	assert_eq!(c[[2,1]], 2.0);
}

#[test]
fn matrix_lup_decomp() {
	let a = Matrix::new(3,3, vec![1.,3.,5.,2.,4.,7.,1.,1.,0.]);

	let (l,u,p) = a.lup_decomp();

	let l_true = vec![1., 0., 0., 0.5, 1., 0., 0.5, -1., 1.];
	let u_true = vec![2., 4., 7., 0., 1., 1.5, 0., 0., -2.];
	let p_true = vec![0., 1., 0., 1., 0., 0., 0., 0., 1.];

	assert_eq!(p.data, p_true);
	assert_eq!(l.data, l_true);
	assert_eq!(u.data, u_true);

	let e = Matrix::<f64>::new(5,5, vec![1.,2.,3.,4.,5.,
									3.,0.,4.,5.,6.,
									2.,1.,2.,3.,4.,
									0.,0.,0.,6.,5.,
									0.,0.,0.,5.,6.]);

	let (l,u,p) = e.lup_decomp();
	let k = p.transpose() * l * u;

	for i in 0..25 {
		println!("{0},{1}", e.data[i], k.data[i]);
		assert_eq!(e.data[i], k.data[i]);
	}
}

#[test]
fn matrix_diag() {
	let a = Matrix::new(3,3, vec![1.,3.,5.,2.,4.,7.,1.,1.,0.]);

	let b = a.is_diag();

	assert!(!b);

	let c = Matrix::new(3,3, vec![1.,0.,0.,0.,2.,0.,0.,0.,3.]);
	let d = c.is_diag();

	assert!(d);
}

#[test]
fn matrix_det() {
	let a = Matrix::new(2,2, vec![2.,3.,1.,2.]);
	let b = a.det();

	assert_eq!(b, 1.);

	let c = Matrix::new(3,3, vec![1.,2.,3.,4.,5.,6.,7.,8.,9.]);
	let d = c.det();

	assert_eq!(d, 0.);

	let e = Matrix::<f64>::new(5,5, vec![1.,2.,3.,4.,5.,
									3.,0.,4.,5.,6.,
									2.,1.,2.,3.,4.,
									0.,0.,0.,6.,5.,
									0.,0.,0.,5.,6.]);

	let f = e.det();

	println!("det is {0}", f);
	let error = abs(f-99.);
	assert!(error < 1e-10);
}

#[test]
fn matrix_solve() {
	let a = Matrix::new(2,2, vec![2.,3.,1.,2.]);

	let y = Vector::new(vec![8., 5.]);

	let x = a.solve(y);

	assert_eq!(x.size, 2);

	assert_eq!(x[0], 1.);
	assert_eq!(x[1], 2.);
}