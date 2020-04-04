fn main() {
    let _x: [i32; 0] = []; // SHOULD PASS
    // let _x: [i32; 0] = [..1]; // SHOULD PASS
    let _x: [std::ops::RangeTo<i32>; 1] = [..1]; // SHOULD FAIL
    let _x: [std::ops::RangeTo<i32>; 2] = [..1, ..(..2)]; // SHOULD PASS
    let _x: [std::ops::RangeTo<i32>; 2] = [..1, .. ..2]; // SHOULD PASS
    // let _x: [std::ops::RangeTo<i32>; 2] = [..1, ..2]; // SHOULD FAIL
    // let _x: [std::ops::RangeTo<i32>; 3] = [..1, ..2, ..3]; // SHOULD FAIL
    let _x: [i32; 1] = [1];
    let _x: [i32; 1] = [1,];
    let _x: [i32; 3] = [1, 2, 3];
    let _x: [i32; 3] = [1, 2, 3,];
    // let _x: [i32; 3] = [1, 2, 3, ..4]; // SHOULD PASS
    let [_x, _y, ..] = [1, 2, 3, 4]; // SHOULD PASS
    // let [_x, _y, ..4] = [1, 2, 3, 4]; // SHOULD FAIL
    // let _x: [i32; 3] = [1, 2, 3, ..4,]; // SHOULD FAIL
    // let _x: [i32; 3] = [1, ..2, 3]; // SHOULD FAIL
}
