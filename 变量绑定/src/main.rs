fn main() {
    // println!("Hello, world!");
    // let an_integer = 1u32;
    // let a_boolean = true;
    // let unit = {};
    // let copied_integer = an_integer;

    // println!("An integer: {:?}", copied_integer);

    // let _unused_variable = 3u32;
    // let noisy_unused_variable = 2u32;

    // mut 可变变量
    // let mut name = "123";
    // println!("Before: {}", name);
    // name = "wenkang";
    // println!("After: {}", name);


    // 作用域和遮蔽
    let long_lived_binding = 1;

    // 这是一个代码块，比 main 函数拥有更小的作用域
    {
        // 此绑定只存在于本代码块
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // 此绑定*遮蔽*了外面的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束

    // 报错！`short_lived_binding` 在此作用域上不存在
    // println!("outer short: {}", short_lived_binding);
    // 改正 ^ 注释掉这行

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样*遮蔽*了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);


    // 4.3 变量可以先声明 后绑定

    // 4.4 冻结

    // 当数据被相同的名称不变地绑定时，它还会冻结（freeze）。在不可变绑定超出作用域之前，无法修改已冻结的数据：

    let mut _mutable_integer = 7i32;

    {
        // 被不可变的 `_mutable_integer` 遮蔽
        let _mutable_integer = _mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        _mutable_integer = 50;
        // 改正 ^ 注释掉上面这行

        // `_mutable_integer` 离开作用域
    }

    // 正常运行！ `_mutable_integer` 在这个作用域没有冻结
    _mutable_integer = 3;

}
