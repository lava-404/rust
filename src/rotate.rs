fn rotate(array :Vec<i32>, k: usize) ->Vec<i32> {

  let mut output: Vec<i32> = vec![];
  let f = array.len() - k;

  for (index, e)  in array.iter().enumerate(){
      if index >= f {
          output.push(*e);
      }

  }
  for (index, e)  in array.iter().enumerate(){
      if index < f{
         output.push(*e);
      }
  }



  output
}



fn main() {
  let output = rotate(vec![1, 2, 3, 4, 5], 2);
  println!("{output:?}")
}