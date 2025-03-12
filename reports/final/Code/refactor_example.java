// Before:
public void processNumbers(List<Integer> nums) {
    int sum = 0;
    for (int n : nums) {
        sum += n;
    }
    System.out.println("Sum is: " + sum);
}

// After:
public void processNumbers(List<Integer> nums) {
    int sum = calculateSum(nums);
    System.out.println("Sum is: " + sum);
}

private int calculateSum(List<Integer> nums) {
    int sum = 0;
    for (int n : nums) {
        sum += n;
    }
    return sum;
}