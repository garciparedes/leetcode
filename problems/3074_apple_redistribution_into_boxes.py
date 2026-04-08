class Solution:
    def minimumBoxes(self, apple: List[int], capacity: List[int]) -> int:
        total_apples = sum(apple)
        capacity.sort(reverse=True)
        current_size = 0
        for i, box_size in enumerate(capacity):
            current_size += box_size
            if total_apples <= current_size:
                return i + 1
        return -1
        
