// start to end
// for i 0 -> 10 {}

// for i 10 <- 0
// println "hello"
// printf ""
// printf ""
// eprintf ""
// for (let i = 10; i > 0; --i) {
//     process.stdout.write("" + i);
// }

// for i 10 <- 0
// for (let i = 10; i > 10; --i) {
//     console.log(i);
// }
/*
    @inline jsfunc a(a, b) `return $a + $b;`
    `return _inline_1 + _inline_2;`

    @inline jsfunc a() `return $a + $b;`
call a(1,2)
let _inline_1 = 1;
let _inline_2 = 2;
    * */
// function a(n) {
//     if (n > 0) {
//         console.log(n);
//         a(n - 1)
//     }
// }
// a(8)
// let _inline_a_1 = 8;
// if (_inline_a_1 > 0) {
//     console.log(_inline_a_1);
//     let _inline_a_1 = _inline_a_1 - 1;
//     if (_inline_a_1 > 0) {
//         console.log(_inline_a_1);
//     }
// }
//
/*
    if (_inline_a_1 > 0) {
        console.log(_inline_a_1);
        a(_inline_a_1 - 1)
    }
let _inline_a_1 = 8;
    if (_inline_a_1 > 0) {
        console.log(_inline_a_1);
        a(_inline_a_1 - 1)
    }
    * */
// jsfunc ret_plus2(a) {return a + 2; }
let inline_arg_b = {
    let inline_arg1 = 3;
    return inline_arg1 + 2
}
