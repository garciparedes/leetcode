from threading import Semaphore

class FizzBuzz:
    def __init__(self, n: int):
        self.n = n
        
        self.done = False        
        
        self.fizz_semaphore = Semaphore(0)
        self.buzz_semaphore = Semaphore(0)
        self.fizzbuzz_semaphore = Semaphore(0)
        self.number_semaphore = Semaphore(1)

    def fizz(self, printFizz: 'Callable[[], None]') -> None:
        while True:
            self.fizz_semaphore.acquire()
            if self.done:
                break
            printFizz()
            self.number_semaphore.release()
            
    def buzz(self, printBuzz: 'Callable[[], None]') -> None:
        while True:
            self.buzz_semaphore.acquire()
            if self.done:
                break
            printBuzz()
            self.number_semaphore.release()

    def fizzbuzz(self, printFizzBuzz: 'Callable[[], None]') -> None: 
        while True:
            self.fizzbuzz_semaphore.acquire()
            if self.done:
                break
            printFizzBuzz()
            self.number_semaphore.release()
            
    # printNumber(x) outputs "x", where x is an integer.
    def number(self, printNumber: 'Callable[[int], None]') -> None:
        for number in range(1, self.n + 1):
            self.number_semaphore.acquire()
            if number % 15 == 0:
                self.fizzbuzz_semaphore.release()
            elif number % 3 == 0:
                self.fizz_semaphore.release()
            elif number % 5 == 0:
                self.buzz_semaphore.release()
            else:
                printNumber(number)
                self.number_semaphore.release()
            
        self.number_semaphore.acquire()
        self.done = True
        self.fizz_semaphore.release()
        self.buzz_semaphore.release()
        self.fizzbuzz_semaphore.release()

