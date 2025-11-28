fn main() {
    let add = |a, b| a + b;
    assert_eq!(add(2,3), 5);

    println!("{:?}", (1..10).filter(|x| x % 2 == 0).collect::<Vec<u32>>());

    let times = 2;
    println!("{:?}", {1..10}.map(|x| x * times).collect::<Vec<u32>>());
    //let arr: [u32: 4] = [1,2,3,4];
    ////println!("{:?}", arr);
    println!("{:?}", {1..10}.map(|x| x * times).collect::<Vec<u32>>());
}
