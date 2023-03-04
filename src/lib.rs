fn bit_or(num : f64)->f64{
    // this function emulates the (num|0) operation in javascript
    // in js, all numbers are 64 bits floating points
    // on a bitwise operation, they are transformed into 32 bits signed integers
    // then converted back into 64 bits javascript numbers
    num as i64 as i32 as f64
}
//alea specific string hash function
struct Mash{
    n:f64
}
impl Mash{
    fn new()->Self{
        Self { n: 4022871197.0 }
    }
    fn next(&mut self, r:&str)->f64{
        let e = 0.02519603282416938;
        for s in r.encode_utf16().collect::<Vec<u16>>(){
            self.n+=s as f64;
            let f = e*self.n-bit_or(e*self.n);
            let t = f* bit_or(e*self.n);
            self.n = 2f64.powi(32) * (t - bit_or(t)) + bit_or(t);
        }
        bit_or(self.n) * 2f64.powi(-32)
    }
}
pub struct Alea{
    s0: f64, //three numbers that determine the internal state of alea
    s1: f64,
    s2: f64,
    x: f64
}
impl Alea {
    pub fn new(seed: String)->Self{
        let mut mash = Mash::new();
        let mut s0 = mash.next(" ");
        let mut s1 = mash.next(" ");
        let mut s2 = mash.next(" ");
        let x = 1.0;
        s0 -= mash.next(&seed);
        s1 -= mash.next(&seed);
        s2 -= mash.next(&seed);
        if s0 < 0.0{
            s0 +=1.0;
        }
        if s1 < 0.0{
            s1 +=1.0;
        }
        if s2 < 0.0{
            s2 +=1.0;
        }
        Self { s0,s1,s2,x}
    }
    pub fn next(&mut self) -> f64{
        let y = self.x * 2f64.powi(-32) + self.s0 * 2091639.0;
        self.s0 = self.s1;
        self.s1 = self.s2;
        self.x = bit_or(y);
        self.s2 = y - self.x;
        self.s2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bit_or_test() {
        assert_eq!(bit_or(0.2), 0.0); //near-zero testing
        assert_eq!(bit_or(-1.8), -1.0);
        assert_eq!(bit_or(1.9), 1.0);

        assert_eq!(bit_or(4294967296.3), 0.0); //u32 min
        assert_eq!(bit_or(4294967295.6), -1.0);
        assert_eq!(bit_or(4294967297.9), 1.0);

        assert_eq!(bit_or(2147483648.9), -2147483648.0); //i32 min
        assert_eq!(bit_or(2147483647.3), 2147483647.0);
        assert_eq!(bit_or(2147483649.1), -2147483647.0);
    }
    #[test]
    fn mash_test(){
        let mut mash = Mash::new();
        assert_eq!(mash.next(""), -0.06335230986587703);
        let mut mash = Mash::new();
        assert_eq!(mash.next(" "), -0.1366710769943893);
        let mut mash = Mash::new();
        assert_eq!(mash.next("frank"), 0.044354382902383804);
        let mut mash = Mash::new();
        assert_eq!(mash.next("cat"), 0.06714190426282585);
        assert_eq!(mash.next("rat"), -0.24548634607344866);
        assert_eq!(mash.next("bat"), 0.05828765174373984);
        assert_eq!(mash.next(" "), 0.03728155279532075);
        assert_eq!(mash.next(" "), 0.32264634780585766);
        assert_eq!(mash.next(" "), -0.356016042875126);
        assert_eq!(mash.next(" "), -0.4360403118189424);
    }
    #[test]
    fn alea_test(){
        let mut a = Alea::new("frank".to_string());
        assert_eq!(a.next(), 0.8080874253064394);
        assert_eq!(a.next(), 0.8366762748919427);
        assert_eq!(a.next(), 0.24404818122275174);
    }
}
/*
This generator implements my variation on Marsaglia's Multiply-with-carry theme,
adapted to javascript's quaint notion of numbers: the carries are exactly the integer parts of Numbers with exactly 32 bits of fractional part.

Such generators depend crucially on their multiplier, a.
It must be less than 221, so that the result of the multiplication fits in 53 bits,
and for an array of n 32-bit numbers, a * 232n - 1 must be a safe prime.
The period is the corresponding Germain prime, a * 232n - 1 - 1.

The one presented here uses n = 3: just 3 32-bit fractions,
which means that one may use three rotating variables without walking through an Array.
(Table lookup is rather expensive, time-wise.) The period is close to 2116, it passes BigCrush,
and it is the fastest javascript PRNG I know that does so.

I expected such generators with any n up to 256 (or even beyond, if one wants monstrously long periods
    and can find the appropriate multipliers) to be faster than those relying on integer arithmetics, which they are.
    But they also turn out to be faster than the lagged Fibonacci generators if one does not insist on 53 bits, much to my surprise.
 */