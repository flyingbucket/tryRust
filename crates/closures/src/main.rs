use closures::{rectangle::Rectangle, shirt};

fn main() {
    // run_shirts();
    run_only_borrow();
    run_mutable_borrow();
    run_move_ownership();
    run_consume_ownership();
    run_return_ownership();
    // run_sort_by_key();
}

fn run_sort_by_key() {
    let mut rectangles = [
        Rectangle { h: 1, w: 2 },
        Rectangle { h: 2, w: 2 },
        Rectangle { h: 3, w: 2 },
        Rectangle { h: 4, w: 2 },
        Rectangle { h: 5, w: 2 },
    ];

    rectangles.sort_by_key(|r| r.w);
    println!("{rectangles:#?}")
}

fn run_shirts() {
    println!("\n===Example of saling shirts===");
    let store = shirt::Inventory {
        shirts: vec![
            shirt::ShirtColor::Red,
            shirt::ShirtColor::Red,
            shirt::ShirtColor::Red,
            shirt::ShirtColor::Red,
            shirt::ShirtColor::Blue,
        ],
    };

    let user1_pref = Some(shirt::ShirtColor::Red);
    let user2_pref: Option<shirt::ShirtColor> = None;

    println!("\nGave away {:?} to user1", store.giveaway(user1_pref));
    println!("\nGave away {:?} to user2", store.giveaway(user2_pref));
}

fn run_only_borrow() {
    println!("\n=== 1. 不可变借用 (Fn) ===");
    let val = String::from("Rust");

    // 闭包只读取 val，因此它自动捕获 &String (不可变借用)
    let only_borrow = || println!("闭包内读取: {}", val);

    // 因为是不可变借用，外部可以同时存在多个读取者
    println!("闭包外读取 (定义后): {}", val);

    only_borrow();
    println!("闭包外读取 (调用后): {}", val);
}

fn run_mutable_borrow() {
    println!("\n=== 2. 可变借用 (FnMut) ===");
    let mut val = String::from("Rust");

    // 闭包修改了 val，因此它捕获 &mut String (可变借用)
    let mut borrow_mutably = || {
        val.push_str(" is Awesome!");
        println!("闭包内修改并读取: {}", val);
    };

    // 在 borrow_mutably 的生命周期内，外部不能读取 val
    // println!("{}", val); // 如果取消注释，编译报错！因为此时 val 被闭包独占借用

    borrow_mutably();

    // 闭包执行完后，借用结束，外部恢复访问权限
    println!("闭包外读取 (调用后): {}", val);
}

fn run_move_ownership() {
    println!("\n=== 3. 移动所有权 (move + Fn/FnOnce) ===");
    let val = String::from("Rust");

    // 使用 move 关键字，显式将所有权移入闭包
    let manual_move = move || {
        println!("闭包内获取所有权: {}", val);
    };

    // 即使闭包还没调用，val 的所有权也已经没了
    // println!("{}", val); // 如果取消注释，编译报错！value borrowed here after move

    manual_move();
    // 移动所有权是一次性的，调用之后所有权也不能自动回到外部作用域
    // println!("{}", val); // 如果取消注释，编译报错！value borrowed here after move
}

fn run_consume_ownership() {
    println!("\n=== 4. 彻底消耗所有权 (FnOnce) ===");
    let val = String::from("Rust");

    // 闭包内部调用了 drop(val)，这会消耗掉 val
    let consume_val = || {
        std::mem::drop(val);
        println!("闭包内：val 已被销毁");
    };

    consume_val();

    // 闭包只能调用一次，且外部无法再访问 val
    // consume_val(); // 如果取消注释，编译报错！因为闭包是 FnOnce
}

fn run_return_ownership() {
    println!("\n=== 5. 先捕获所有权再返回所有权并在外部scope赋值给变量===.");
    let val = String::from("Rust");

    // 闭包接收所有权，执行完后再把它返回
    let return_move = move || {
        println!("闭包内：{}", val);
        val // 将所有权移出闭包
    };

    // 通过返回值，重新在外部 scope 锚定所有权
    let val = return_move();

    println!("外部重新获得所有权: {}", val); // 现在可以用了！
}
