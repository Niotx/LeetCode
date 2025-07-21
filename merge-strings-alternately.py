class Solution(object):
    def mergeAlternately(self, word1, word2):
        """
        :type word1: str
        :type word2: str
        :rtype: str
        """
        num1 = len(word1)
        num2 = len(word2)

        mx = max(num1, num2)
        mi = min(num1, num2)

        res = ""

        if word1 == "":
            return word2

        if word2 == "":
            return word1


        for i in range(mi): 
            res = res + word1[i]+word2[i]

        if len(word1) > mi:
            res = res + word1[mi:len(word1)]
        if len(word2) > mi: 
            res = res + word2[mi:len(word2)]
        return res
        
