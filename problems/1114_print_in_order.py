from threading import Semaphore

class Foo:
    
    def __init__(self):
        self._second_sem = Semaphore(0)
        self._third_sem = Semaphore(0)
        
    def first(self, printFirst: 'Callable[[], None]') -> None:
        
        # printFirst() outputs "first". Do not change or remove this line.
        printFirst()
        self._second_sem.release()

    def second(self, printSecond: 'Callable[[], None]') -> None:
        
        # printSecond() outputs "second". Do not change or remove this line.
        self._second_sem.acquire()
        printSecond()
        self._second_sem.release()
        self._third_sem.release()

    def third(self, printThird: 'Callable[[], None]') -> None:
        
        # printThird() outputs "third". Do not change or remove this line.
        self._third_sem.acquire()
        printThird()
        self._third_sem.acquire()

