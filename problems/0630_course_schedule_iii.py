from operator import itemgetter
from heapq import heappush, heappop


class Solution:
    def scheduleCourse(self, courses: List[List[int]]) -> int:
        heap = list()
        start = 0
        for duration, latest in sorted(courses, key=itemgetter(1)):
            start += duration
            heappush(heap, -duration)
            while start > latest:
                start += heappop(heap)
                
        return len(heap)
