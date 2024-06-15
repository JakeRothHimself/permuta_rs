from permuta_rs import test
from permuta import *
import collections
import time

def test_py():
    # pi = Perm((2, 5, 0, 3, 6, 4, 7, 1))

    # print(pi._pattern_details())

    # print(list(pi.left_floor_and_ceiling()))

    # deq: Deque[Tuple[int, int]] = collections.deque()
    # smallest, biggest = -1, -1
    # for idx, val in enumerate(pi):
    #     if idx == 0:
    #         deq.append((val, idx))
    #         smallest, biggest = val, val
    #     elif val < smallest:
    #         # Rotate until smallest val is at front
    #         while deq[0][0] != smallest:
    #             deq.rotate(-1)
    #         deq.appendleft((val, idx))
    #         smallest = val
    #     elif val > biggest:
    #         # Rotate until biggest val is at end
    #         while deq[-1][0] != biggest:
    #             deq.rotate(-1)
    #         deq.append((val, idx))
    #         biggest = val
    #     else:
    #         while not deq[-1][0] <= val <= deq[0][0]:
    #             deq.rotate(1)
    #         deq.appendleft((val, idx))
    # print(deq)

    # start = time.perf_counter()
    # test()
    # end = time.perf_counter()
    # print(end- start)

    # start = time.perf_counter()
    # ret = list(Perm((2, 0, 1)).occurrences_in(Perm((5, 3, 0, 4, 2, 1))))
    # end = time.perf_counter()
    # print(end- start)
    
    # print(ret)
    # test()

    # c = Av(Basis(Perm((0,2,1))))
    # start = time.perf_counter()
    # ret = len(list(c.of_length(15)))
    # print(ret)
    # end = time.perf_counter()
    # print(end- start)

    start = time.perf_counter()
    test()
    end = time.perf_counter()
    print(end- start)

test_py()

