fn main() {
    println!("\n\nHomework-2 #task_3: Sum function");

    //1.在范围内求和
    let sum_1:[u32; 7]=[2345,28989,222,323,123,88998,3232];
    match sum(&sum_1){
        Some(result) => println!("Calc result: {}", result),
        None => println!("Over the limitation of u32 0~4,294,967,295"),
    }

    //2.超出了范围
    let sum_2:[u32; 5]=[4294967295,28989,222,323,123];
    match sum(&sum_2){
        Some(result) => println!("Calc result: {}", result),
        None => println!("Over the limitation of u32 0~4,294,967,295"),
    }
}

fn sum(arr: &[u32]) -> Option<u32> {
    let max: u32 = 4_294_967_295;
    println!("U32 Max: {}",max);

    let mut i=0;
    let mut result:u32=0;
    while i<arr.len() {
        if result.checked_add(arr[i]).is_none(){
           return None;
        } 

        result += arr[i];
        i += 1;
    }
    return Some(result);
}

//Unit test, run by `cargo test`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_normal() {
        let arr:[u32; 7]=[1,2,3,4,5,6,7];
        let expected = 28;
        assert_eq!(sum(&arr), Some(expected));
    }

    #[test]
    fn test_sum_overflow() {
        let arr:[u32; 3]=[4294967295,3,2];
        let expected = None;
        assert_eq!(sum(&arr), expected);
    }
}