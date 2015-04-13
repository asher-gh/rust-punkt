use num::Float;

/// From the original paper.
pub fn dunning_log_likelihood(count_a: f64, count_b: f64, count_ab: f64, n: f64) -> f64 {
  let p1 = count_b / n;
  let p2 = 0.99;
  let nullh = count_ab * p1.ln() + (count_a - count_ab) * (1.0 - p1).ln();
  let alth = count_ab * p2.ln() + (count_a - count_ab) * (1.0 - p2).ln();

  -2.0 * (nullh - alth)
}

/// From the original paper.
pub fn col_log_likelihood(count_a: f64, count_b: f64, count_ab: f64, n: f64) -> f64 {
  let p = count_b / n;
  let p1 = count_ab / count_a;
  let p2 = (count_b - count_ab) / (n - count_a);

  let s1 = count_ab * p.ln() + (count_a - count_ab) * (1.0 - p).ln();
  let s2 = (count_b - count_ab) * p.ln() + 
           (n - count_a - count_b + count_ab) * 
           (1.0 - p).ln();
  let s3 = if count_a == count_ab {
    0f64
  } else {
    count_ab * p1.ln() + (count_a - count_ab) * (1.0 - p1).ln()
  };
  let s4 = if count_b == count_ab {
    0f64
  } else {
    (count_b - count_ab) * 
    p2.ln() + 
    (n - count_a - count_b + count_ab) * 
    (1.0 - p2).ln()
  };

  -2.0 * (s1 + s2 - s3 - s4)
}
