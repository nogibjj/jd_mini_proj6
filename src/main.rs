use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

fn main() {
    let y = vec![1., 2., 3., 4., 5.];
    let x1 = vec![5., 4., 3., 2., 1.];
    let x2 = vec![729.53, 439.0367, 42.054, 1., 0.];
    let x3 = vec![258.589, 616.297, 215.061, 498.361, 0.];
    let data = vec![("Y", y), ("X1", x1), ("X2", x2), ("X3", x3)];
    let data = RegressionDataBuilder::new().build_from(data).expect("warning");


    let formula = "Y ~ X1 + X2 + X3";
    let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula(formula)
        .fit();
    let model_clone = model.expect("warning");
    let parameters: Vec<_> = model_clone.iter_parameter_pairs().collect();
    let pvalues: Vec<_> = model_clone.iter_p_value_pairs().collect();
    let standard_errors: Vec<_> = model_clone.iter_se_pairs().collect();

    println!("Parameters: {:?}", parameters);
    println!("P-values: {:?}", pvalues);
    println!("Standard errors: {:?}", standard_errors);
}
