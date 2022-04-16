class Solution:
    def memLeak(self, memory1: int, memory2: int) -> List[int]: 
        leak = 1
        while memory1 >= leak or memory2 >= leak:
            if memory1 >= memory2:
                memory1 -= leak
            else:   
                memory2 -= leak
            leak += 1
        
        return [leak, memory1, memory2]
        
        
    
