pub struct S<'a>{
    pub r: &'a i32,
}

pub struct D<'a>{
    pub d: S<'a>,
}

fn main() {
    let num = 10;
    let _s: S<'_> = S {r: &num};
    let _d: D<'_> = D {d: _s};

    /*
     * when borrowed, can't even move 
     */
    // let v = vec![4, 8, 19, 27, 34, 10];
    // let r = &v;
    // let aside = v;  // move vector to aside
    // r[0];                     // bad: uses `v`, which is now uninitialized
}