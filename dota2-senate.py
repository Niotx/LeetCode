from collections import deque

class Solution:
    def predictPartyVictory(self, senate: str) -> str:
        n = len(senate)
        queueR = deque()
        queueD = deque()
        for i,s in enumerate(senate):
            if s == 'R':
                queueR.append(i)
            else:
                queueD.append(i)
        while queueR and queueD:
            r = queueR.popleft()
            d = queueD.popleft()
            if r < d:
                queueR.append(r + n)
            else:
                queueD.append(d + n)
        return "Radiant" if queueR else "Dire"
