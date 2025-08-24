from collections import deque

class RecentCounter:

    def __init__(self):
        self.requests = deque()  # queue to store request times

    def ping(self, t: int) -> int:
        self.requests.append(t)  # add new request
        # remove requests older than t - 3000
        while self.requests[0] < t - 3000:
            self.requests.popleft()
        return len(self.requests)
