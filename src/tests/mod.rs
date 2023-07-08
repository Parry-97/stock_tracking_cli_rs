#![allow(non_snake_case)]
use tokio::join;

use crate::types::{AsyncStockSignal, MaxPrice, MinPrice, PriceDifference, WindowedSMA};

#[tokio::test]
async fn test_PriceDifference_calculate() {
    let signal = PriceDifference {};
    assert_eq!(signal.calculate(&[]).await, None);
    assert_eq!(signal.calculate(&[1.0]).await, Some((0.0, 0.0)));
    assert_eq!(signal.calculate(&[1.0, 0.0]).await, Some((-1.0, -1.0)));
    assert_eq!(
        signal
            .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
            .await,
        Some((8.0, 4.0))
    );
    assert_eq!(
        signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
        Some((1.0, 1.0))
    );
}

#[tokio::test]
async fn test_parallel_PriceDifference_calculate() {
    let signal = PriceDifference {};
    let (empty_series, single_value_series, multiple_vals) = join!(
        signal.calculate(&[]),
        signal.calculate(&[1.0]),
        signal.calculate(&[1.0, 0.0])
    );

    assert_eq!(empty_series, None);
    assert_eq!(single_value_series, Some((0.0, 0.0)));
    assert_eq!(multiple_vals, Some((-1.0, -1.0)));
    // assert_eq!(
    //     signal
    //         .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
    //         .await,
    //     Some((8.0, 4.0))
    // );
    // assert_eq!(
    //     signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
    //     Some((1.0, 1.0))
    // );
}

#[tokio::test]
async fn test_parallel_MinPrice_calculate() {
    let signal = MinPrice {};
    let (empty_series, single_value_series, multiple_vals) = join!(
        signal.calculate(&[]),
        signal.calculate(&[1.0]),
        signal.calculate(&[1.0, 0.0])
    );

    assert_eq!(empty_series, None);
    assert_eq!(single_value_series, Some(1.0));
    assert_eq!(multiple_vals, Some(0.0));
    // assert_eq!(signal.calculate(&[]).await, None);
    // assert_eq!(signal.calculate(&[1.0]).await, Some(1.0));
    // assert_eq!(signal.calculate(&[1.0, 0.0]).await, Some(0.0));
    // assert_eq!(
    //     signal
    //         .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
    //         .await,
    //     Some(1.0)
    // );
    // assert_eq!(
    //     signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
    //     Some(0.0)
    // );
}
#[tokio::test]
async fn test_MinPrice_calculate() {
    let signal = MinPrice {};
    assert_eq!(signal.calculate(&[]).await, None);
    assert_eq!(signal.calculate(&[1.0]).await, Some(1.0));
    assert_eq!(signal.calculate(&[1.0, 0.0]).await, Some(0.0));
    assert_eq!(
        signal
            .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
            .await,
        Some(1.0)
    );
    assert_eq!(
        signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
        Some(0.0)
    );
}

#[tokio::test]
async fn test_MaxPrice_calculate() {
    let signal = MaxPrice {};
    assert_eq!(signal.calculate(&[]).await, None);
    assert_eq!(signal.calculate(&[1.0]).await, Some(1.0));
    assert_eq!(signal.calculate(&[1.0, 0.0]).await, Some(1.0));
    assert_eq!(
        signal
            .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
            .await,
        Some(10.0)
    );
    assert_eq!(
        signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
        Some(6.0)
    );
}

#[tokio::test]
async fn test_WindowedSMA_calculate() {
    let series = vec![2.0, 4.5, 5.3, 6.5, 4.7];

    let signal = WindowedSMA { window_size: 3 };
    assert_eq!(
        signal.calculate(&series).await,
        Some(vec![3.9333333333333336, 5.433333333333334, 5.5])
    );

    let signal = WindowedSMA { window_size: 5 };
    assert_eq!(signal.calculate(&series).await, Some(vec![4.6]));

    let signal = WindowedSMA { window_size: 10 };
    assert_eq!(signal.calculate(&series).await, Some(vec![]));
}
