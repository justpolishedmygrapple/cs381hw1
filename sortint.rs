
fn insertion_sort<Int: Ord + Copy>(arr: &mut [Int]){

    for i in 1..arr.len(){

        let mut j = i;

        while j > 0 && arr[j] < arr[j-1]{

            let temp = arr[j-1];

            arr[j-1] = arr[j];

            arr[j] = temp;

            j = j - 1;


        }


    }

}


fn main(){


    let mut unsorted_array = [12,19,1,192,7,0,-1,-28,1928];

   
    println!("Before sorting, your array is: {:?}", unsorted_array);

    insertion_sort(&mut unsorted_array);


    println!("After sorting, your array is {:?}", unsorted_array);



}
