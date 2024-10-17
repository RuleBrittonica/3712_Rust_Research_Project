/// The function the user wants to refactor.
/// The sleep calls are simulating long, complex operations.
/// The example is contrived, but the refactoring is applicable to real-world code.
async fn compute_sum_async<T>(items: &[T]) -> T
where
    T: std::ops::Add<Output = T> + Copy + Send,
{
    // First Extraction Block
    let sum = {
        let sleep = task::sleep(std::time::Duration::from_millis(10)).await;
        items[0]
    };

    for &item in &items[1..] {
        // Second Extraction Block
        sum = {
            task::sleep(std::time::Duration::from_millis(10)).await;
            sum + item
        };
    }
    sum
}

/// The output of the refactoring.
/// Note how the generic type T is still appropriately constrained.
/// And that the async blocks have been extracted into separate functions, with
/// the appropriate constraints.
async fn compute_sum_async<T>(items: &[T]) -> T
where
    T: std::ops::Add<Output = T> + Copy + Send,
{
    let mut sum = initialize_sum(items).await;

    for &item in &items[1..] {
        sum = add_items_async(sum, item).await;
    }
    sum
}

async fn initialize_sum<T>(items: &[T]) -> T
where
    T: Copy,
{
    task::sleep(std::time::Duration::from_millis(10)).await;
    items[0]
}

async fn add_items_async<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    task::sleep(std::time::Duration::from_millis(10)).await;
    a + b
}