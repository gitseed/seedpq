fn main() {
    dbg!(even_array::<5>());
}

fn even_array<const N: usize>() -> [usize; N] {
    core::array::from_fn(|x| x * 2)
}

// pub fn fetch_one<const N: usize>(&self) -> [&[u8]; N] {
//     let foo: [usize; N] = (0..N).map(|x| x * 2).collect();

// }
