/*
Copyright (c) 1970-2003, Wm. Randolph Franklin

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or
sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

1. Redistributions of source code must retain the above
copyright notice, this list of conditions and the following
disclaimers.
2. Redistributions in binary form must reproduce the above
copyright notice in the documentation and/or other materials
provided with the distribution.
3. The name of W. Randolph Franklin may not be used to
endorse or promote products derived from this Software without
specific prior written permission.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

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

//! # pnpoly
//!
//! `pnpoly` is a simple port of W. Randolph Franklin's [PNPOLY](https://wrf.ecse.rpi.edu//Research/Short_Notes/pnpoly.html) algorithm to Rust.

#[cfg(test)]
mod pnpoly_tests;

use std::cmp::PartialOrd;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

fn _pnpoly
    <T: PartialOrd + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy>
    (nvert: usize, vertx: &Vec<T>, verty: &Vec<T>, testx: T, testy: T)
    -> bool
{
    let mut c: bool = false;
    let mut j: usize = nvert - 1 as usize;
    for i in 0..nvert {
        if ((verty[i] > testy) != (verty[j] > testy)) &&
           (testx < (vertx[j] - vertx[i]) * (testy - verty[i]) / (verty[j] - verty[i]) + vertx[i]) {
            c = !c;
        }
        j = i;
    }
    c
}

/// Performs a *point-included-in-polygon* test
///
/// # Examples
///
/// ```
/// use pnpoly::pnpoly;
///
/// let vertices = vec![
///     vec![ 1.0, 1.0 ],
///     vec![ 1.0, 2.0 ],
///     vec![ 2.0, 2.0 ],
///     vec![ 2.0, 1.0 ]
/// ];
///
/// let point = vec![ 1.2, 1.9 ];
///
/// let included = pnpoly(&vertices, &point);
/// ```
///
/// # Panics
///
///   + if any *leaf* vectors do not consist of a coordinate **pair** (is not a vector of length 2)
///   + if any *leaf* vector contains anything but a numeric or an unsigned numeric type, fx u32
///
pub fn pnpoly
    <T: PartialOrd + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy>
    (vertices: &Vec<Vec<T>>, point: &Vec<T>)
    -> bool
{
    _pnpoly(
        vertices.len(),
        &vertices.iter().map(|ref latlng| latlng[0]).collect::<Vec<T>>(),
        &vertices.iter().map(|ref latlng| latlng[1]).collect::<Vec<T>>(),
        point[0],
        point[1]
    )
}
