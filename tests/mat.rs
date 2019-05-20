use std::mem::transmute;

use opencv::core::{self, Mat, Scalar, Size, Vec2b};
use opencv::types::VectorOfMat;

const PIXEL: &[u8] = include_bytes!("../pixel.png");

#[test]
fn mat_for_rows_and_cols() {
    let typ = core::CV_8UC3;
    let mat = unsafe { Mat::new_rows_cols(400, 300, typ) }.unwrap();
    assert_eq!(mat.typ().unwrap(), typ);
    assert_eq!(mat.size().unwrap(), Size::new(300, 400));
    assert_eq!(core::CV_8U, mat.depth().unwrap());
    assert_eq!(3, mat.channels().unwrap());
}

#[test]
fn mat_at() {
    let mut mat = Mat::new_rows_cols_with_default(100, 100, core::CV_32F, Scalar::all(0.)).unwrap();
    assert_eq!(*mat.at_2d::<f32>(0, 0).unwrap(), 0.);
    *mat.at_2d_mut::<f32>(0, 0).unwrap() = 1.;
    assert_eq!(*mat.at_2d::<f32>(0, 0).unwrap(), 1.);

    if let Ok(..) = mat.at::<i32>(0) {
        assert!(false, "different types");
    }

    let row = mat.at_row::<f32>(0).unwrap();
    assert_eq!(row.len(), 100);
    assert_eq!(row[0], 1.);

    let row = mat.at_row_mut::<f32>(1).unwrap();
    row[0..4].copy_from_slice(&[10., 20., 30., 40.]);

    let data = mat.data::<f32>().unwrap();
    assert_eq!(data[0], 1.);
    assert_eq!(data[100], 10.);
    assert_eq!(data[103], 40.);

    // todo unallocated Mat, zero sized Mat
}

#[test]
fn mat_vec() {
    let s = vec![
        vec![1.0f32, 2., 3.],
        vec![4., 5., 6.],
        vec![7., 8., 9.],
    ];

    let mat = Mat::from_slice_2d(&s).unwrap();
    assert_eq!(mat.size().unwrap(), core::Size { width: 3, height: 3 });
    assert_eq!(*mat.at_2d::<f32>(1, 1).unwrap(), 5.);

    let v = mat.to_vec_2d::<f32>().unwrap();
    assert_eq!(s, v);
}

#[test]
fn mat_operations() {
    {
        let mut src = VectorOfMat::new();
        src.push(Mat::new_rows_cols_with_default(1, 3, core::CV_8U, Scalar::all(1.)).unwrap());
        src.push(Mat::new_rows_cols_with_default(1, 3, core::CV_8U, Scalar::all(2.)).unwrap());
        let mut dst = Mat::default();
        core::merge(&src, &mut dst).unwrap();
        assert_eq!(dst.typ().unwrap(), core::CV_8UC2);
        assert_eq!(dst.at_2d::<Vec2b>(0, 1).unwrap()[0], 1);
        assert_eq!(dst.at_2d::<Vec2b>(0, 2).unwrap()[1], 2);
    }
}


#[test]
fn mat_from_data() {
    let mut bytes = PIXEL.to_vec();
    let src = Mat::new_rows_cols_with_data(1, PIXEL.len() as _, core::CV_8U, unsafe { transmute(bytes.as_mut_ptr()) }, core::Mat_AUTO_STEP).unwrap();
    assert_eq!(src.size().unwrap(), Size::new(PIXEL.len() as _, 1));
    assert_eq!(src.total().unwrap(), PIXEL.len());
    let row = src.at_row::<u8>(0).unwrap();
    assert_eq!(row[0], 0x89);
    assert_eq!(row[11], 0x0D);
    assert_eq!(row[89], 0x82);
}
