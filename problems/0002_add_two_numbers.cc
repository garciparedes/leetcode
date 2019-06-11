#include <iostream>
#include <sstream>

#include <string>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Definition for singly-linked list.
 */
struct ListNode {
    int val;
    ListNode *next;

    explicit ListNode(int x) : val(x), next(nullptr) {}

    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2, bool carry = false) {
        int val = l1->val + l2->val + carry;
        if ((carry = val > 9)) {
            val -= 10;
        }

        ListNode *result = new ListNode(val);

        if (l1->next != nullptr && l2->next != nullptr) {
            result->next = addTwoNumbers(l1->next, l2->next, carry);
        } else if (l1->next != nullptr) {
            result->next = addTwoNumbers(l1->next, new ListNode(carry));
        } else if (l2->next != nullptr) {
            result->next = addTwoNumbers(l2->next, new ListNode(carry));
        } else if (carry) {
            result->next = new ListNode(1);
        }
        return result;
    }
};

int main() {

    ListNode l1(7);
    ListNode l2(7, new ListNode(3));

    ListNode *solution = Solution().addTwoNumbers(&l1, &l2);

    stringstream ss;
    do {
        ss << solution->val;
        solution = solution->next;
    } while (solution != nullptr);
    string s = ss.str();
    reverse(s.begin(), s.end());
    cout << s << '\n';

    return 0;

}