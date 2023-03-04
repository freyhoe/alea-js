fn bit_or(num : f64)->f64{
    // this function emulates the (num|0) operation in javascript
    // in js, all numbers are 64 bits floating points
    // on a bitwise operation, they are transformed into 32 bits signed integers
    // then converted back into 64 bits javascript numbers
    num as i64 as i32 as f64
}
/// Alea specific string hash function, faithful to javascript outputs.
/// Normally used privately, but can be used alone.
/// 
/// # Example
/// 
/// ```
/// let mash = Mash::new();
/// mash.hash("string to hash")
/// mash.hash("second string to hash")
/// ```
#[derive(Copy, Clone)]
pub struct Mash{
    n:f64
}
impl Mash{
    pub fn new()->Self{
        Self { n: 4022871197.0 }
    }
    /// Hashes a string.
    /// Function alters Mash state:
    /// Hashing the same string twice produces different results.
    pub fn hash(&mut self, r:&str)->f64{
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
/// Seedable random number generator, faithful to javascript.
/// # Examples
/// 
/// ```
/// let mut a = Alea::new("frank".to_string());
/// assert_eq!(a.random(), 0.8080874253064394);
/// assert_eq!(a.random(), 0.8366762748919427);
/// assert_eq!(a.random(), 0.24404818122275174);
/// let mut a = Alea::new("frank".to_string());
/// assert_eq!(a.uint32(), 3470709064);
/// ```
#[derive(Copy, Clone)]
pub struct Alea{
    s0: f64, //three numbers that determine the internal state of alea.
    s1: f64,
    s2: f64,
    x: f64
}
impl Alea {
    /// Initializes the random number generator with a string seed.
    pub fn new(seed: String)->Self{
        let mut mash = Mash::new();
        let mut s0 = mash.hash(" ");
        let mut s1 = mash.hash(" ");
        let mut s2 = mash.hash(" ");
        let x = 1.0;
        s0 -= mash.hash(&seed);
        s1 -= mash.hash(&seed);
        s2 -= mash.hash(&seed);
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
    /// Returns a random f64 value.
    pub fn random(&mut self) -> f64{
        let y = self.x * 2f64.powi(-32) + self.s0 * 2091639.0;
        self.s0 = self.s1;
        self.s1 = self.s2;
        self.x = bit_or(y);
        self.s2 = y - self.x;
        self.s2
    }
    /// Returns a random u32 value.
    pub fn uint32(&mut self)-> u32{
        (self.random() * 2f64.powi(32)) as u32
    }
}

struct MashFast{
    n:f64
}
impl MashFast{
    pub fn new()->Self{
        Self { n: 4022871197.0 }
    }
    pub fn hash(&mut self, r:&str)->f64{
        for s in r.encode_utf16().collect::<Vec<u16>>(){
            self.n += s as f64;
            let mut hash: f64 = self.n * 0.02519603282416938;
            self.n = hash.trunc();
            hash -= self.n;
            hash *= self.n;
            self.n = hash.trunc();
            hash -= self.n;
            self.n += (hash * 2f64.powi(32)).trunc();
        }
        (self.n)* 2f64.powi(-32)
    }
}
/// Seedable random number generator.
/// This version is more performant, but could vary from javascript with extreme values.
#[derive(Copy, Clone)]
pub struct AleaFast{
    s0: f64, //three numbers that determine the internal state of alea
    s1: f64,
    s2: f64,
    x: f64
}
impl AleaFast {
    /// Initializes the random number generator with a string seed.
    pub fn new(seed: String)->Self{
        let mut mash = MashFast::new();
        let mut s0 = mash.hash(" ");
        let mut s1 = mash.hash(" ");
        let mut s2 = mash.hash(" ");
        let x = 1.0;
        s0 -= mash.hash(&seed);
        s1 -= mash.hash(&seed);
        s2 -= mash.hash(&seed);
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
    /// Returns a random f64 value.
    pub fn random(&mut self) -> f64{
        let y = self.x * 2f64.powi(-32) + self.s0 * 2091639.0;
        self.s0 = self.s1;
        self.s1 = self.s2;
        self.x = y.trunc();
        self.s2 = y - self.x;
        self.s2
    }
    /// Returns a random u32 value.
    pub fn uint32(&mut self)-> u32{
        (self.random() * 2f64.powi(32)) as u32
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
        assert_eq!(mash.hash(""), -0.06335230986587703);
        let mut mash = Mash::new();
        assert_eq!(mash.hash(" "), -0.1366710769943893);
        let mut mash = Mash::new();
        assert_eq!(mash.hash("frank"), 0.044354382902383804);
        let mut mash = Mash::new();
        assert_eq!(mash.hash("cat"), 0.06714190426282585);
        assert_eq!(mash.hash("rat"), -0.24548634607344866);
        assert_eq!(mash.hash("bat"), 0.05828765174373984);
        assert_eq!(mash.hash(" "), 0.03728155279532075);
        assert_eq!(mash.hash(" "), 0.32264634780585766);
        assert_eq!(mash.hash(" "), -0.356016042875126);
        assert_eq!(mash.hash(" "), -0.4360403118189424);
    }
    #[test]
    fn alea_test(){
        let mut a = Alea::new("frank".to_string());
        assert_eq!(a.random(), 0.8080874253064394);
        assert_eq!(a.random(), 0.8366762748919427);
        assert_eq!(a.random(), 0.24404818122275174);
        let mut a = Alea::new("frank".to_string());
        assert_eq!(a.uint32(), 3470709064);
    }
    #[test]
    fn alea_fast_test(){
        let mut a = AleaFast::new("frank".to_string());
        assert_eq!(a.random(), 0.8080874253064394);
        assert_eq!(a.random(), 0.8366762748919427);
        assert_eq!(a.random(), 0.24404818122275174);
        let mut a = AleaFast::new("frank".to_string());
        assert_eq!(a.uint32(), 3470709064);
    }
}