use std::{f32};

pub fn getQuadraticPolyRoot(quadterm: f32, linearterm: f32, constant: f32) -> [Option<f32>; 2]
{
    let temp = Some((linearterm).powf(2.0) - 4.0 * quadterm * constant);
    let denominator =  2.0 * quadterm;
    match temp
    {
        Some(0.0) => [ Some(-linearterm / denominator), None ],
        Some(x) if x > 0.0 =>
            {
                let tempsquared = x.powf(0.5) ;
                [ Some((-linearterm - tempsquared) / denominator), Some((-linearterm + tempsquared) / denominator) ]
            },
        _ => [None, None],
    }
}

#[cfg(test)]
mod PolynomialTest
{
    use super::*;
    use approx::{assert_relative_eq};
    use std::f32;

    #[test]
    fn checkTwoRoots()
    {
        let qt = 5.0;
        let lt = 3.0;
        let cons = -2.0;
        let res = getQuadraticPolyRoot(qt, lt, cons);
        assert_relative_eq!(res[0].unwrap(), -1.0, epsilon=f32::EPSILON);
        assert_relative_eq!(res[1].unwrap(), 0.4, epsilon=f32::EPSILON);
    }

    #[test]
    fn checkNoRoot()
    {
        let qt = 6.0;
        let lt = 3.0;
        let cons = 1.0;
        let res = getQuadraticPolyRoot(qt, lt, cons);
        assert_eq!(res, [None, None]);
    }
}