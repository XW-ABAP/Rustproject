fn main() {
    // println!("Hello, world!");
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 " 1" ，5 个空格后面连着 1 。
    println!("{number:>width$}", number=1, width=6);

    // 你可以在数字左边补 0 。下面语句输出 "000001" 。
    println!("{number:>0width$}", number=1, width=6);
    // println! 会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "Bond", "James);");
    // 改正 ^ 补上漏掉的参数："James"
    // 创建一个包含单个 `i32` 的结构体（structure ） 。命名为 `Structure` 。
    #[allow(dead_code)]
    struct Structure(i32);
    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    // println!("This struct `{}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。
}

