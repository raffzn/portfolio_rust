fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.1, 8.0, 10.0]; // leve ruído para testar métricas

    match regressao_linear(&x, &y) {
        Ok((a, b)) => {
            println!("Coeficientes: a = {:.4}, b = {:.4}", a, b);
            let y_pred = prever(&x, a, b);
            println!("Previsões: {:?}", y_pred);

            match calcular_r2(&y, &y_pred) {
                Ok(r2) => println!("R² = {:.4}", r2),
                Err(e) => println!("Erro ao calcular R²: {}", e),
            }

            match calcular_mse(&y, &y_pred) {
                Ok(mse) => println!("MSE = {:.4}", mse),
                Err(e) => println!("Erro ao calcular MSE: {}", e),
            }
        }
        Err(e) => println!("Erro: {}", e),
    }
}

/// Implementa a regressão linear simples (reta y = ax + b)
pub fn regressao_linear(x: &[f64], y: &[f64]) -> Result<(f64, f64), &'static str> {
    if x.len() != y.len() || x.is_empty() {
        return Err("Vetores de entrada inválidos");
    }

    let n = x.len() as f64;
    let soma_x: f64 = x.iter().sum();
    let soma_y: f64 = y.iter().sum();
    let soma_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let soma_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let denominador = n * soma_x2 - soma_x * soma_x;
    if denominador == 0.0 {
        return Err("Denominador zero: não é possível calcular");
    }

    let a = (n * soma_xy - soma_x * soma_y) / denominador;
    let b = (soma_y - a * soma_x) / n;

    Ok((a, b))
}

/// Calcula previsões usando os coeficientes da regressão linear
pub fn prever(x: &[f64], coef_a: f64, coef_b: f64) -> Vec<f64> {
    x.iter().map(|xi| coef_a * xi + coef_b).collect()
}

/// Calcula o coeficiente de determinação (R²)
pub fn calcular_r2(y_real: &[f64], y_pred: &[f64]) -> Result<f64, &'static str> {
    if y_real.len() != y_pred.len() || y_real.is_empty() {
        return Err("Entradas inválidas");
    }

    let media_y = y_real.iter().sum::<f64>() / y_real.len() as f64;
    let ss_total: f64 = y_real.iter().map(|y| (y - media_y).powi(2)).sum();
    let ss_res: f64 = y_real.iter().zip(y_pred).map(|(y, y_hat)| (y - y_hat).powi(2)).sum();

    Ok(1.0 - (ss_res / ss_total))
}

/// Calcula o erro quadrático médio (MSE)
pub fn calcular_mse(y_real: &[f64], y_pred: &[f64]) -> Result<f64, &'static str> {
    if y_real.len() != y_pred.len() || y_real.is_empty() {
        return Err("Entradas inválidas");
    }

    let mse: f64 = y_real.iter().zip(y_pred).map(|(y, y_hat)| (y - y_hat).powi(2)).sum::<f64>() / y_real.len() as f64;

    Ok(mse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regressao_linear_perfeita() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (a, b) = regressao_linear(&x, &y).unwrap();
        assert!((a - 2.0).abs() < 1e-6);
        assert!((b - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_prever() {
        let x = vec![1.0, 2.0, 3.0];
        let (a, b) = (2.0, 1.0);
        let y_pred = prever(&x, a, b);
        assert_eq!(y_pred, vec![3.0, 5.0, 7.0]);
    }

    #[test]
    fn test_r2_mse() {
        let y_real = vec![2.0, 4.0, 6.0, 8.0];
        let y_pred = vec![2.0, 4.1, 5.9, 8.2];
        let r2 = calcular_r2(&y_real, &y_pred).unwrap();
        let mse = calcular_mse(&y_real, &y_pred).unwrap();
        assert!(r2 > 0.99);
        assert!(mse < 0.05);
    }

    #[test]
    fn test_erros_de_entrada() {
        let vazio: Vec<f64> = vec![];
        assert!(regressao_linear(&vazio, &vazio).is_err());
        assert!(calcular_r2(&vazio, &vazio).is_err());
        assert!(calcular_mse(&vazio, &vazio).is_err());
    }
}
