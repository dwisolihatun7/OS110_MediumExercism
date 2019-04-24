
#[derive(Debug, PartialEq)]
pub enum Classification {
    Abundant, // jumlah dari faktornya sama dengan bilangan yang difaktorkan
    Perfect, // jumlah faktornya lebih besar dari bilangan yang difaktorkan
    Deficient, // jumlah faktornya lebih kecil dari bilangan yang difaktorkan
}

use Classification::*;
pub fn classify(num: u64) -> Option<Classification> {
   // unimplemented!("classify {}", num);
   if num == 0 {
       return None;
       }
   let sum: u64 = (1..num).filter(|value|num % value ==0).sum();
   match sum{
       sum if sum==num=>Some(Perfect),
       sum if sum>num=>Some(Abundant),
       _=>Some(Deficient),
   }
    
}


   
