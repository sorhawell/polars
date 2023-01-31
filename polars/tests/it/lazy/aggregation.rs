use super::*;

#[test]
fn test_lazy_df_aggregations() {
    let df = load_df();

    assert!(df
        .clone()
        .lazy()
        .min()
        .collect()
        .unwrap()
        .frame_equal_missing(&df.min()));
    assert!(df
        .clone()
        .lazy()
        .median()
        .collect()
        .unwrap()
        .frame_equal_missing(&df.median()));
    assert!(df
        .clone()
        .lazy()
        .quantile(lit(0.5), QuantileInterpolOptions::default())
        .collect()
        .unwrap()
        .frame_equal_missing(
            &df.quantile(0.5, QuantileInterpolOptions::default())
                .unwrap()
        ));
}

#[test]
#[cfg(feature = "temporal")]
fn test_lazy_agg() {
    let s0 = DateChunked::parse_from_str_slice(
        "date",
        &[
            "2020-08-21",
            "2020-08-21",
            "2020-08-22",
            "2020-08-23",
            "2020-08-22",
        ],
        "%Y-%m-%d",
    )
    .into_series();
    let s1 = Series::new("temp", [20, 10, 7, 9, 1].as_ref());
    let s2 = Series::new("rain", [0.2, 0.1, 0.3, 0.1, 0.01].as_ref());
    let df = DataFrame::new(vec![s0, s1, s2]).unwrap();

    let lf = df
        .lazy()
        .groupby([col("date")])
        .agg([
            col("rain").min().alias("min"),
            col("rain").sum().alias("sum"),
            col("rain")
                .quantile(lit(0.5), QuantileInterpolOptions::default())
                .alias("median_rain"),
        ])
        .sort("date", Default::default());

    let new = lf.collect().unwrap();
    let min = new.column("min").unwrap();
    assert_eq!(min, &Series::new("min", [0.1f64, 0.01, 0.1]));
}
