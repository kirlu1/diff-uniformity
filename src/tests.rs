


#[cfg(test)]
mod poly_test {
    use crate::*;

    #[test]
    fn add_test() {
        let lhs = Poly(vec![1,1,1,1],2);
        let rhs = Poly(vec![0,1,1],2);
        let result = lhs + rhs;
        assert_eq!(result, Poly(vec![1,0,0,1], 2));
    }

    #[test]
    fn mul_test() {
        let lhs = Poly(vec![1,1,1],2);
        let rhs = Poly(vec![0,1,0],2);
        let result = (lhs.clone() * rhs.clone()).prune();
        assert_eq!(result, Poly(vec![0,1,1,1], 2));
        let rev_result = rhs.clone() * lhs.clone();
        assert_eq!(rev_result, Poly(vec![0,1,1,1], 2));

        let product = Poly(vec![1,2],3) * Poly(vec![1,0,1], 3);
        assert_eq!(product, Poly(vec![1,2,1,2],3));
    }

    #[test]
    fn prune_test() {
        let pol = Poly(vec![1,0,1,1,0,0,0,0],2);
        let pruned = pol.prune();
        assert_eq!(pruned, Poly(vec![1,0,1,1],2))
    }

    #[test]
    fn reduce_test() {
        let irr_pol = Poly(vec![2,2,1], 3);
        let product = Poly(vec![1,2],3) * Poly(vec![1,0,1], 3);

        assert_eq!(product.reduced(&irr_pol), Poly(vec![1,1],3));
    }
}

