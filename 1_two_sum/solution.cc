#include <vector>
#include <unordered_map>

using std::unordered_map;
using std::vector;
vector<int> twoSum(vector<int> &nums, int target)
{
  unordered_map<int, int> heap;
  for (int i = 0; i < nums.size(); i++)
  {
    int r = target - nums[i];
    if (heap.count(r))
      return {heap[r], i};
    else
      heap[nums[i]] = i;
  }
  return {-1, -1};
}