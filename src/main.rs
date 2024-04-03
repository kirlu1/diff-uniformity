use std::iter::{repeat, once};

mod tests;

fn main() {
    let irr_pol = Poly(vec![1,1,0,1], 2);
    let q1 = diff_uniformity(
        irr_pol.clone().field_elements(),
        |x| (x.clone() * x),
        |x| x.reduced(&irr_pol)
    );
    println!("Task 2.1.4: {}", q1);
    
    let irr_pol = Poly(vec![2,2,1], 3);
    let q5 = diff_uniformity(
        irr_pol.clone().field_elements(),
        |x| (x.clone() * x),
        |x| x.reduced(&irr_pol)
    );
    
    println!("Task 2.5: {}", q5);
}

// The zero-element is assumed to be the first of the field iterator
fn diff_uniformity<T, I>(field : I, map : impl Fn(T) -> T, close : impl Fn(T) -> T) -> usize
where 
T : std::ops::Add<Output=T> + std::ops::Sub<Output=T> + Eq + Clone,
I : Iterator<Item=T> + Clone
{
    let mut max = 0;

    for b in field.clone() {
        for a in field.clone().skip(1) { // skip 0
            let mut count = 0;

            for x in field.clone() {
                let lhs = map(x.clone()+a.clone()) - map(x.clone());
                if close(lhs) == b.clone() {
                    count+=1;
                }
            }
            max = count.max(max);
        }
    }
    max
}


type PolType = Vec<i32>;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Poly(PolType, i32);

#[derive(Clone)]
struct PolyIterator {
    irr_pol : Poly,
    gen_pol : Poly,
    count : usize,
}

impl Poly {
    fn prune(self) -> Poly {
        let basefield = self.1;
        let nonzeroes: usize = self.0.iter()
            .enumerate()
            .fold(0, |acc, (idx, e)| {
                if e!=&0 {
                    idx+1
                } else {
                    acc
                }
            });

        Poly(self.0.into_iter().take(nonzeroes).collect(), basefield)
    }

    // naive implementation
    fn reduced(self, irr_pol : &Poly) -> Poly {
        let basefield = self.1;
        let mut polynomial = self;
        let irr_degree = irr_pol.0.len()-1;

        while polynomial.0.len() > irr_degree {
            let highest = polynomial.0.last().unwrap().clone();
            if highest == 0 {
            polynomial.0.pop();
                continue
            }

            let mut v = vec![0; polynomial.0.len()-irr_degree-1];
            v.push(highest);

            let reduction = Poly(v, basefield) * irr_pol.clone();
            polynomial = polynomial - reduction;

            let x = polynomial.0.pop().unwrap();
            if x != 0 {
                dbg!(polynomial);
                panic!("wtf, x: {}", x)
            }
        }


        polynomial
    }

    fn field_elements(self) -> PolyIterator {
        let base_field = self.1;
        let degree = self.0.len()-1;
        PolyIterator {
            irr_pol : self,
            gen_pol : Poly(vec![0; degree], base_field),
            count : 0,
        }
    }
}

impl std::ops::Add for Poly {
    type Output = Poly;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ascending = vec![self.0, rhs.0];
        let base_field = self.1;
        ascending.sort_by_key(Vec::len); // Determine lower and higher degree

        let mut longer = ascending.pop().unwrap();
        let shorter = ascending.pop().unwrap();

        for (idx, e) in shorter.into_iter().enumerate() {
            longer[idx]=(longer[idx]+e).rem_euclid(base_field) ;
        }
        Poly(longer, base_field)
    }
}

impl std::ops::AddAssign for Poly {
    fn add_assign(&mut self, rhs: Self) {
        let temp = std::mem::replace(self, Poly(vec![], self.1));
        let result = temp + rhs;
        let _ = std::mem::replace(self, result);
    }
}

impl std::ops::Mul for Poly {
    type Output = Poly;

    fn mul(self, rhs: Self) -> Self::Output {
        let base_field = self.1;
        let deg = self.0.len()+rhs.0.len()-1;
        let mut product : Vec<i32> = vec![0; deg];

        for (deg_i, i) in self.0.iter().enumerate() {
            for (deg_j, j) in rhs.0.iter().enumerate() {
                product[deg_i+deg_j]+=i*j;
            }
        }

        for x in product.iter_mut() {
            *x = x.rem_euclid(base_field);
        }
        Poly(product, base_field).prune()
    }
}

impl std::ops::Sub for Poly {
    type Output = Poly;

    fn sub(self, mut rhs: Self) -> Self::Output {
        for x in rhs.0.iter_mut() {
            *x*=-1;
        }
        self+rhs // modulo handled in add
    }
}

impl Iterator for PolyIterator {
    type Item = Poly;

    fn next(&mut self) -> Option<Self::Item> {
        let basefield = self.irr_pol.1;
        if self.count == basefield.pow(self.gen_pol.0.len() as u32) as usize{
            return None
        }
        self.count+=1;
        let value = self.gen_pol.clone();

        // Modify generator
        for val in self.gen_pol.0.iter_mut() {
            if val < &mut (self.gen_pol.1-1) {
                *val+=1;
                break;
            }
            *val = 0;
        }
        //
        Some(value)
    }
}
