fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push("1");
    v.push("2");
    v.push("3");

    let removed_element = v.remove(1);
    let removed_element = v.pop();

    f(&mut v);

    let mut vec = Vec::with_capacity(5);
    vec.resize(5, 0);
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    assert_eq!(None, v.get(3));
}


fn f(v: &mut Vec<u32>) {
    v.clear();
    v.resize(10, 0);
}


//  ๐·°(৹˃̵﹏˂̵৹)°·๐