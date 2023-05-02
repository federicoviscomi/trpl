fn main() {
    println!("Hello, world!");
    page_32();
    page_39();
    page_40();
    page_42();
    page_53();
    page_55();
}

fn page_55() {
    let mut counter = 0;
    let result = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{counter} {result}");

    for element in [2, 6, 876] {
        print!("{element} ");
    }
    println!("");

    for num in (1..11).rev() {
        print!("{num} ");
    }
    println!("");
}

fn page_53() {
    let x = 5;
    let y: i32 = if x < 10 {
        0
    } else {
        10
    };
    println!("{x} {y}");
}

fn page_42() {
    let months: [&str; 3] = ["Jan", "Feb", "Mar"];
    println!("{:?} {:?}", months, months[2]);
}

fn page_32() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn page_39() {
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 67.5 / 32.2;
    let trunc = -5 / 3;
    let rem = 43 % 3;
    println!("{sum} {diff} {prod} {quot} {trunc} {rem}");
}

fn page_40() {
    let c = 'z';
    let z: char = 'z';
    let tup: (i32, f64, char) = (234, 34.3, 's');
    let (x, y, wayoming) = tup;
    let a = [1, 2, 3];
    println!("{c} {z} {x} {y} {wayoming} {:?} {} {:?}", tup, tup.0, a);
}
