fn main() {
    let _x: [i32; 0] = [];
    // let _x: [i32; 0] = [..1]; // SHOULD PASS
    let _x: [i32; 1] = [1];
    let _x: [i32; 1] = [1,];
    let _x: [i32; 3] = [1, 2, 3];
    let _x: [i32; 3] = [1, 2, 3,];
    let _x: [i32; 3] = [1, 2, 3, ..4];
    // let _x: [i32; 3] = [1, 2, 3, ..4,]; // SHOULD FAIL
    // let _x: [i32; 3] = [1, ..2, 3]; // SHOULD FAIL
}
