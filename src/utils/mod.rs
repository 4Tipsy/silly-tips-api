
use rand::Rng;
use std::iter;



pub fn gen_num_hash(length: u8) -> String {
  let mut rng = rand::thread_rng();
  let charset: Vec<char> = (b'a'..=b'z')
      .chain(b'0'..=b'9')
      .map(|x| x as char)
      .collect();

  iter::repeat(())
      .take(length as usize)
      .map(|_| charset[rng.gen_range(0..charset.len())])
      .collect()
}








pub fn gen_simple_hash(length: u8) -> String {
    let mut rng = rand::thread_rng();
    let charset: Vec<char> = (b'a'..=b'z')
        .chain(b'0'..=b'9')
        .map(|x| x as char)
        .collect();

    (0..length as usize)
        .map(|_| charset[rng.gen_range(0..charset.len())])
        .collect()
}