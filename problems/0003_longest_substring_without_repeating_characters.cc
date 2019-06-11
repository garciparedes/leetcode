#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int lengthOfLongestSubstring(const string &s) {
        unordered_map<int, string::const_iterator> indexes;

        int max_substring = 0;

        auto left = s.begin() - 1;
        for (auto right = s.begin(); right < s.end(); right++) {

            if (indexes.find(*right) != indexes.end() && indexes[*right] >= left) {
                left = indexes[*right];
            }

            max_substring = max(max_substring, static_cast<int>(right - left));
            indexes[*right] = right;
        }
        return max_substring;

    }

};

int main() {

    cout << Solution().lengthOfLongestSubstring("abcabcbb") << '\n';
    cout << Solution().lengthOfLongestSubstring("bbbbb") << '\n';
    cout << Solution().lengthOfLongestSubstring("pwwkew") << '\n';
    cout << Solution().lengthOfLongestSubstring("dvdf") << '\n';
    cout << Solution().lengthOfLongestSubstring(" ") << '\n';
    cout << Solution().lengthOfLongestSubstring("") << '\n';
    cout << Solution().lengthOfLongestSubstring("abba") << '\n';


    return 0;
}