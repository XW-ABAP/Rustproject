// test 1
// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// test 2
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// test 3
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    // 1
    // let spaces = "   ";      // 此时 spaces 是字符串类型 (&str)
    // let spaces = spaces.len(); // 此时创建了一个全新的 spaces 变量，它是数字类型 (usize)
    // 这段话的重点是： 当你第二次写下 let spaces 时，你实际上是在内存中创建了一个全新的变量，
    // 只是“借用”了之前的名字，把旧的变量给“遮蔽”起来了。因为它是全新的，所以它的数据类型可以完全不同。


    // 2. mut（可变变量）不允许改变“类型”
    // 如果我们用 mut 来写，逻辑就完全不同了。mut 代表这个变量的值可以改变，但它的类型在第一次声明时就已经焊死了。

    // Rust
    // let mut spaces = "   "; // 声明 spaces 为可变变量，类型被固定为字符串 (&str)
    // spaces = spaces.len();  // 试图把一个数字赋值给字符串变量 -> 报错！

}