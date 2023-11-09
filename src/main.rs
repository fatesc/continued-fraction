fn fmt_continued(exact: bool, first_num: f64, sequence: &mut [f64]) -> String {
    let sequence_cycle = sequence.iter().map(|n| n.to_string()).cycle();
    let sequence_len = sequence.len();

    return format!("[ {}; {} ]", first_num as i32,
        if exact {
            sequence_cycle.take(sequence_len.min(11)).collect::<Vec<_>>().join(", ") + if sequence_len > 11 { ", ..." } else { "" }
        } else if sequence_len < 11 {
            sequence_cycle.take(sequence_len * 2).collect::<Vec<_>>().join(", ") + ", ..."
        } else {
            sequence_cycle.take(11).collect::<Vec<_>>().join(", ")
        }
    );
}

fn continued(num: f64) -> String {
    let mut integer_part = num.floor();
    let first_integer = integer_part;
    let mut fractional_part = num % 1.;
    let mut reciprocal = 1. / fractional_part;
    let mut sequence = Vec::new();
    let mut len = 0;

    while (fractional_part * 10000.).floor() != 0. {
        integer_part = reciprocal.floor();
        fractional_part = reciprocal % 1.;
        reciprocal = 1. / fractional_part;

        sequence.push(integer_part);
        len += 1;

        if len % 11 == 0 {
            if &sequence[0..len / 11] == &sequence[len / 11..len] {
                println!("repeating sequence found!");
                return fmt_continued(false, first_integer, &mut sequence[0..len / 11]);
            }
        }
    }
    return fmt_continued(true, first_integer, &mut sequence);
}

fn main() {
    let exact = 415. / 93.;
    println!("{}", continued(exact)); // [4; 2, 6, 7]

    let pattern = 19_f64.sqrt();
    println!("{}", continued(pattern)); // [4;2,1,3,1,2,8,2,1,3,1,2,8,...]

    let pi = std::f64::consts::PI;
    println!("{}", continued(pi)); // [3;7,15,1,292,1,1,1,2,1,3,1,...]

    let e = std::f64::consts::E;
    println!("{}", continued(e)); // [2;1,2,1,1,4,1,1,6,1,1,8,...]

    let phi = 1.618033988749894848204586834365638118_f64;
    println!("{}", continued(phi)); // [1;1,1,1,1,1,1,1,1,1,1,1,...]

    let y = 0.57721566490153286060651209008240243104215933593992_f64;
    println!("{}", continued(y)); // [0;1,1,2,1,2,1,4,3,13,5,1,...]

    let root2 = std::f64::consts::SQRT_2;
    println!("{}", continued(root2));
}
