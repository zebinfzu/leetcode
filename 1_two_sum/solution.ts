function twoSum(nums: number[], target: number): number[] {
  const heap: Map<number, number> = new Map();
  const ans: number[] = [];
  nums.forEach((val, idx) => {
    if (heap.has(target - val))
      ans.push(idx, heap.get(target - val) as number);
    else heap.set(val, idx);
  })
  return ans;
};