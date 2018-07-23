#[test]
fn internal () {
    let nvert: usize = 4;
    let vertx: Vec<f64> = vec![ 1.0, 1.0, 2.0, 2.0 ];
    let verty: Vec<f64> = vec![ 1.0, 2.0, 2.0, 1.0 ];
    let testx: f64 = 1.2;
    let testy: f64 = 1.9;
    assert_eq!(super::_pnpoly(nvert, &vertx, &verty, testx, testy), true);
}

#[test]
fn pnpoly_pt1 () {
    let vertices: Vec<Vec<f64>> = vec![
        vec![ 1.0, 1.0 ],
        vec![ 1.0, 2.0 ],
        vec![ 2.0, 2.0 ],
        vec![ 2.0, 1.0 ]
    ];
    let point: Vec<f64> = vec![ 1.2, 1.9 ];
    assert_eq!(super::pnpoly(&vertices, &point), true);
}

#[test]
fn pnpoly_pt2 () {
    let vertices: Vec<Vec<f64>> = vec![
        vec![ 1.0, 1.0 ],
        vec![ 1.0, 2.0 ],
        vec![ 2.0, 2.0 ],
        vec![ 2.0, 1.0 ]
    ];
    let point: Vec<f64> = vec![ 3.2, 4.9 ];
    assert_eq!(super::pnpoly(&vertices, &point), false);
}

#[test]
fn pnpoly_pt3 () {
    let vertices: Vec<Vec<f32>> = vec![
        vec![ 1.0, 1.0 ],
        vec![ 1.0, 4.0 ],
        vec![ 4.0, 4.0 ],
        vec![ 4.0, 1.0 ]
    ];
    let point: Vec<f32> = vec![ 2.0, 2.0 ];
    assert_eq!(super::pnpoly(&vertices, &point), true);
}

#[test]
fn pnpoly_pt4 () {
    let vertices: Vec<Vec<i32>> = vec![
        vec![ 1, 1 ],
        vec![ 1, 4 ],
        vec![ 4, 4 ],
        vec![ 4, 1 ]
    ];
    let point: Vec<i32> = vec![ 2, 2 ];
    assert_eq!(super::pnpoly(&vertices, &point), true);
}

#[test]
fn pnpoly_pt5 () {
    let vertices: Vec<Vec<i64>> = vec![
        vec![ 1, 1 ],
        vec![ 1, 4 ],
        vec![ 4, 4 ],
        vec![ 4, 1 ]
    ];
    let point: Vec<i64> = vec![ 2, 2 ];
    assert_eq!(super::pnpoly(&vertices, &point), true);
}

#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn panic_pt1 () {
    let vertices: Vec<Vec<u32>> = vec![
        vec![ 1, 1 ],
        vec![ 1, 4 ],
        vec![ 4, 4 ],
        vec![ 4, 1 ]
    ];
    let point: Vec<u32> = vec![ 2, 2 ];
    assert_eq!(super::pnpoly(&vertices, &point), true);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn panic_pt2 () {
    let vertices: Vec<Vec<f64>> = vec![
        vec![ 1.0, 1.0 ],
        vec![ 1.0, 2.0 ],
        vec![ 2.0 ],
        vec![ 2.0, 1.0 ]
    ];
    let point: Vec<f64> = vec![ 3.2, 4.9 ];
    super::pnpoly(&vertices, &point);
    ()
}
