fn main() {
    let mut a = vec![90, 60, 89];
    a.push(100); //班里来个赵铁柱同学，可以用push追加数据
    println!("{:?}", a);

    let b = a;
    println!("{:?}", b);

    a = vec![200, 300, 600]; //还可以给a赋值，因为此时a还被drop
    println!("{:?}", a);
}
