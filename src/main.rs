use rand::Rng;
use plotpy::{generate3d, Contour, Plot, StrError};

fn main() {

    let diff_rand = 0.5;
    let seed_rand = 0.5;
    let len: isize = 50;

    let mut rng = rand::thread_rng();
    let mut t: &mut Vec<Vec<f64>> = & mut vec![vec![0.0;len as usize];len as usize];
    let mut tab : [[f64;9];9] = [[0.0; 9]; 9]; // Initialize tab with default value
    let mut n1: f64 = rng.gen();

    t[0][0] = n1;
    //let _ = std::mem::replace(&mut t[0][0], n1);

    for i in 0..len {
        for j in 0..len {
            if i == 0 && j == 0 {

            } else {
                n1 = rng.gen();
                let dx = moyenne(t, i, j) * (1.0 - ((n1 * diff_rand) + (0.0 - (diff_rand / 2.0))));
                t[i as usize][j as usize] = dx;
            }
        }
    }

    print_tab(t);
    let ma = max(t);
    println!("{ma}");

    let mi = min(t);
    println!("{mi}");

    for i in t{
        for j in i{
            
        }
    }


}


fn moyenne(table: &mut Vec<Vec<f64>>, x: isize, y: isize) -> f64 {
    let mut sum = 0.0;
    let mut n = 0;

    for i in -1..=1{
        for j in -1..=1{
            let x1 = x + i;
            let y1 = y + j;

            let len1 = table.len() as isize;

            if x1 >=0 && x1 <=len1-1 &&
                y1 >=0 && y1 <=len1-1 && !(x1 == x && y1 == y) {
                    if table[x1 as usize][y1 as usize] == 0.0 {

                    }else{
                    sum += table[x1 as usize][y1 as usize];
                    n+=1;
                }
            }
        }
    }
    return sum/n as f64;
}

fn print_tab(table:&mut Vec<Vec<f64>>) {
    for i in table{
        print!("[");
        for j in i {
            print!("{j},");
        }
        print!("],");
    }
}

fn max(table:&mut Vec<Vec<f64>>)-> f64 {
    let mut max: f64 = f64::MIN;
    for i in table{
        for j in i {
            if j>= &mut max{
                max = *j;
            }
        }
    }
    return max;
}

fn min(table:&mut Vec<Vec<f64>>)-> f64 {
    let mut min: f64 = f64::MAX;
    for i in table{
        for j in i {
            if j<= &mut min{
                min = *j;
            }
        }
    }
    return min;
}