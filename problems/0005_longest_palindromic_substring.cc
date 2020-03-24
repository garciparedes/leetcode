
class Cell {
 public:
  int value() {
    if (left == -1 || right == -1) {
      return -1;
    }
    return right - left;
  }

  int offset() {
    if (value() == 0) {
      return 1;
    }
    return value() + 1;
  }

  int left = -1;
  int right = -1;
};

class Solution {
 public:
  Cell *longest_palindrome(int left, int right, vector<vector<Cell>> &table, const string &text) {

    if (table[left][right].value() != -1) {
      return &table[left][right];
    }


    if (!(left < right)) {
      table[left][right].left = left;
      table[left][right].right = right;
      return &table[left][right];
    }

    auto full_pruning = longest_palindrome(left + 1, right - 1, table, text);
    if (text[left] == text[right] && left + 1 == full_pruning->left && right - 1 == full_pruning->right) {
      table[left][right].left = left;
      table[left][right].right = right;
    }

    auto left_pruning = longest_palindrome(left + 1, right, table, text);
    auto right_pruning = longest_palindrome(left, right - 1, table, text);

    if (table[left][right].value() < left_pruning->value()) {
      table[left][right] = *left_pruning;
    }
    if (table[left][right].value() < right_pruning->value()) {
      table[left][right] = *right_pruning;
    }
    return &table[left][right];
  }

  string longestPalindrome(const string &text) {
    if (text.size() < 2) {
      return text;
    }

    vector<vector<Cell>> table(text.size(), vector<Cell>(text.size()));
    auto best_pair = longest_palindrome(0, text.size() - 1, table, text);
    return text.substr(best_pair->left, best_pair->offset());

  }
};
