#include <iostream>
#include <vector>

using namespace std;


class Solution {
public:

    double findMedianSortedArrays(const vector<int> &nums1, const vector<int> &nums2) {
        auto it_left_1 = nums1.begin();
        auto it_right_1 = nums1.rbegin();

        auto it_left_2 = nums2.begin();
        auto it_right_2 = nums2.rbegin();

        int left;
        int right;
        do {

            if (!(it_left_2 < nums2.end()) || (it_left_1 < nums1.end() && *it_left_1 < *it_left_2)) {
                left = *it_left_1;
                it_left_1 += 1;
            } else {
                left = *it_left_2;
                it_left_2 += 1;
            }

            if (!(it_right_1 < nums1.rend()) || (it_right_2 < nums2.rend() && *it_right_1 < *it_right_2)) {
                right = *it_right_2;
                it_right_2 += 1;
            } else {
                right = *it_right_1;
                it_right_1 += 1;
            }
        } while (left < right);


        return (left + right) / 2.0;
    }
};


int main() {

    cout << Solution().findMedianSortedArrays({1, 3}, {}) << '\n';
    cout << Solution().findMedianSortedArrays({3}, {-2, -1}) << '\n';
    cout << Solution().findMedianSortedArrays({7}, {7}) << '\n';
    cout << Solution().findMedianSortedArrays({1, 3, 4, 6}, {7}) << '\n';
    cout << Solution().findMedianSortedArrays({1, 3}, {2}) << '\n';
    cout << Solution().findMedianSortedArrays({1, 2}, {3, 4}) << '\n';

    return 0;
}