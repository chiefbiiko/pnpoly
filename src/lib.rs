/*
int pnpoly(int nvert, float *vertx, float *verty, float testx, float testy)
{
  int i, j, c = 0;
  for (i = 0, j = nvert-1; i < nvert; j = i++) {
    if ( ((verty[i]>testy) != (verty[j]>testy)) &&
	 (testx < (vertx[j]-vertx[i]) * (testy-verty[i]) / (verty[j]-verty[i]) + vertx[i]) )
       c = !c;
  }
  return c;
}
*/

fn pnpoly (nvert: usize, vertx: &Vec<f64>, verty: &Vec<f64>, testx: f64, testy: f64) -> bool {
    let mut c: bool = false;
    let mut j: usize = nvert - 1 as usize;
    for i in 0..nvert {
        if ((verty[i] > testy) != (verty[j] > testy)) &&
           (testx < (vertx[j] - vertx[i]) * (testy - verty[i]) / (verty[j] - verty[i]) + vertx[i]) {
            c = !c;
        }
        j = i;
    }
    return c;
}

fn pnpoly_wrapper (vertices: &Vec<Vec<f64>>, point: &Vec<f64>) -> bool { // whats up wit da ref???
    let nvert: usize = vertices.len();
    let vertx: Vec<f64> = vertices.iter().map(|ref latlng| latlng[0]).collect::<Vec<f64>>();
    let verty: Vec<f64> = vertices.iter().map(|ref latlng| latlng[1]).collect::<Vec<f64>>();
    let testx: f64 = point[0];
    let testy: f64 = point[1];
    return pnpoly(nvert, &vertx, &verty, testx, testy);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic () {
        let nvert: usize = 4;
        let vertx: Vec<f64> = vec![ 1.0, 1.0, 2.0, 2.0 ];
        let verty: Vec<f64> = vec![ 1.0, 2.0, 2.0, 1.0 ];
        let testx: f64 = 1.2;
        let testy: f64 = 1.9;
        assert_eq!(super::pnpoly(nvert, &vertx, &verty, testx, testy), true);
    }
    #[test]
    fn test_wrapper_pt1 () {
        let vertices = vec![
            vec![ 1.0, 1.0 ],
            vec![ 1.0, 2.0 ],
            vec![ 2.0, 2.0 ],
            vec![ 2.0, 1.0 ]
        ];
        let point = vec![ 1.2, 1.9 ];
        assert_eq!(super::pnpoly_wrapper(&vertices, &point), true);
    }
    #[test]
    fn test_wrapper_pt2 () {
        let vertices = vec![
            vec![ 1.0, 1.0 ],
            vec![ 1.0, 2.0 ],
            vec![ 2.0, 2.0 ],
            vec![ 2.0, 1.0 ]
        ];
        let point = vec![ 3.2, 4.9  ];
        assert_eq!(super::pnpoly_wrapper(&vertices, &point), false);
    }
}
