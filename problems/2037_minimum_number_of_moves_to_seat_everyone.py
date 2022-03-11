class Solution:
    def minMovesToSeat(self, seats: List[int], students: List[int]) -> int:
        seats.sort()
        students.sort()
        
        ans = 0
        for seat, student in zip(seats, students):
            ans += abs(seat - student)
            
        return ans
