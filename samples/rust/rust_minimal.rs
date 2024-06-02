// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.

use kernel::prelude::*;
use alloc::string::String;
use kernel::{str::CString, fmt};
use core::ops::Deref;

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                let _ = temp_vec.push($x, GFP_KERNEL);
            )*
            temp_vec
        }
    };
}

#[derive(Debug)]
struct MyVec<T> (Vec<T>);

impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T:Clone> Clone for MyVec<T> {
    fn clone(&self) -> Self {
        let mut vec = Vec::new();
        vec.try_reserve(self.len()).unwrap();
        for elem in self.iter() {
            vec.push(elem.clone(), GFP_KERNEL);
        }
        MyVec(vec)
    }
}


impl<T> FromIterator<T> for MyVec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> MyVec<T> {
        let mut vec = Vec::new();
        for i in iter {
            let _ = vec.push(i, GFP_KERNEL);
        }
        MyVec(vec)
    }
}

// TODO: marker trait exact_size_hint
module! {
    type: RustMinimal,
    name: "rust_minimal",
    author: "Rust for Linux Contributors",
    description: "Rust minimal sample",
    license: "GPL",
}

struct RustMinimal {
    numbers: Vec<i32>,
}

impl kernel::Module for RustMinimal {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust minimal sample (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        let numbers: Vec<i32> = vec![1, 2, 3];
        pr_info!("numbers: {:?}\n", numbers);
        let numbers_mapped:MyVec<i32> = numbers.iter().map(|x| x*x).collect();
        pr_info!("numbers: {:?}\n", numbers_mapped.clone());

        let s: &CStr = CStr::from_bytes_with_nul(b"Hello World!\0").unwrap();
        let s2 = s.to_str().unwrap();
        pr_info!("s: {:?}, ss: {:?}\n", s, s2.split("N").collect::<MyVec<&str>>());

        Ok(RustMinimal { numbers })
    }
}

impl Drop for RustMinimal {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust minimal sample (exit)\n");
    }
}
