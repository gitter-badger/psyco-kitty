
// dynamic programming

fn main() {
    let mut total_result: i32 = 0;
    let mut result: i32;
    let mut count: i32 = 1;
    loop {
        result = fib(count);
        if result > 4000000 {
            break;
        }
        if result % 2 == 0 {
            total_result += result;
            println!("{}",result)
        }
        count += 1;
    }

    println!("{}",total_result);

}

static mut mem_table : [i32; 10000] = [-1;10000];

fn fib(i: i32) -> i32{
    unsafe {
        if mem_table[i as usize] != -1 {
            let temp = mem_table[i as usize];
            return temp;
        }
    } 

    if i == 1 {
        return 1;
    }
    if i==2 {
        return 2;
    }
    unsafe{
        mem_table[i as usize] = fib(i-2) + fib(i-1);

        mem_table[i as usize]
    }

}
