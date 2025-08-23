#include <iostream>
#include <string>
#include <cctype>
using namespace std;

class Solution {
public:
    int idx = 0;  // global pointer for recursion
    
    string decodeString(string s) {
        idx = 0;
        return rec(s);
    }

    string rec(string &s) {
        string result = "";
        while (idx < s.size() && s[idx] != ']') {
            if (isdigit(s[idx])) {
                // 1. get number (could be multiple digits)
                int count = 0;
                while (idx < s.size() && isdigit(s[idx])) {
                    count = count * 10 + (s[idx] - '0');
                    idx++;
                }

                // 2. skip '['
                idx++; 

                // 3. decode inside brackets
                string inside = rec(s);

                // 4. skip ']'
                idx++;

                // 5. repeat 'inside' count times
                for (int k = 0; k < count; k++) {
                    result += inside;
                }
            } else {
                result += s[idx];
                idx++;
            }
        }
        return result;
    }
};
