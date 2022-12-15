//
// mod.rs
// Copyright (C) 2022 tinix <tinix@archlinux>
// Distributed under terms of the MIT license.
//

pub trait Encryptable { 
    fn encrypt(&self) -> String;
}

pub mod rot13;
