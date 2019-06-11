#include <iostream>

#include<vector>
#include<unordered_map>

using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int> &nums, int target) {
        unordered_map<int, int> processed;

        for (int i = 0; i < nums.size(); i++) {
            int value = nums[i];

            auto it = processed.find(target - value);

            if (it != processed.end()) {
                return {it->second, i};
            }

            processed.insert(make_pair(value, i));
        }
        return {-1, -1};

    }
};

int main() {

    vector<int> nums = {2, 7, 11, 15};
    int target = 9;

    vector<int> solution = Solution().twoSum(nums, target);

    for (auto value : solution) {
        cout << value << ' ';
    }
    cout << '\n';

    return 0;

}