

use rand::Rng;
use num_bigint::{BigInt, BigUint, RandBigInt};


type Big=BigUint;
fn div(n:&Big)->(Big,Big){
    let zero=Big::from(0u32);
    let one=Big::from(1u32);
    let two=Big::from(2u32);
    let mut d: BigUint=n-&one;
    let mut s=Big::from(0u32);
    while &d%&two==zero{
        d=&d/&two;
        s=s+&one;
    }
    (d,s)
}
fn miller_rab(n:&Big,k:u32)->bool{
    let mut zero=Big::from(0u32);
    let one=Big::from(1u32);
    let two=Big::from(2u32);
    let three=Big::from(3u32);
    let (d,s)=div(n);
    let mut rng=rand::thread_rng();
    let n_minus_two=n-&two;
    let mut nn=n.clone();
    if n<=&one{return false;}
    if n==&two || n==&three{return true;}
    if n%&three==zero || n%Big::from(5u32) == zero{return false;}
    for _ in 0..k{
        let mut a=rng.gen_biguint_range(&two, &n_minus_two);

        let mut c=mod_power(&mut a, & d, &nn);
        if c==one || c==n-&one{
            continue;
        }
        let mut is_composite=true;
        let mut i =Big::from(0u32);
        while i<&s-&one{
            c=(&c*&c)%n;
            if c ==n-&one{
                is_composite=false;
                break;
            }
            i+=&one;

       }
    if is_composite{
        return false;
    }

    }
    true
}
fn mod_power(b:&mut Big,e:&Big,m:&Big)->Big{
    if *m ==BigUint::from(1u32){
        return BigUint::from(0u32);
    }
    let zero=Big::from(0u32);
    let one=Big::from(1u32);
    let two=Big::from(2u32);
    let mut result:Big=Big::from(1u32);
    let mut exp=e.clone();
    *b=&*b%m;
    while exp>zero{
        if &exp%&two==one{
            result=(result*(&*b))%m;
        }
        
        *b=(&*b)*(&*b)%m;
        exp=&exp/&two;
    }

    result
}
fn main() {
    let mut rng=rand::thread_rng();
    let  mut primes:Vec<Big>=Vec::with_capacity(100);
    while primes.len()<100{
        let mut condidate=rng.gen_biguint(2048);
        condidate.set_bit(2047,true);
        condidate.set_bit(0,true);
        let result=miller_rab(&condidate,40);
        
        if result{
            
            primes.push(condidate);
        }

    }
    println!("{:?}",primes);
    
    let mut a=Big::from(33u32);
    let b=Big::from(33u32);
    a=a+b;

}
