from permuta_rs import test
from permuta import *
import collections

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

    res = test([2, 0, 1])
    print(res)
    

test_py()

