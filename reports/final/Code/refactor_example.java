// Before:
public void doComplicatedWork(List<Integer> nums) {
    int sum = 0;
    for (int n : nums) { // Consider this as a 
        sum += n;        // Lengthy piece of 
    }                    // Business logic
    System.out.println("Sum is: " + sum);
}

// After:
public void doComplicatedWork(List<Integer> nums) {
    int sum = doSmallPartOfWork(nums); // Logic is now reusable
    System.out.println("Sum is: " + sum);
}
// This function can be unit tested on its own
private int doSmallPartOfWork(List<Integer> nums) {
    int sum = 0;         
    for (int n : nums) {
        sum += n;
    }
    return sum;
}