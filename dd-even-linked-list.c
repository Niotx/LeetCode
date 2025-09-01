/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode* oddEvenList(struct ListNode* head) {
    if (!head || !head->next) return head;
    struct ListNode* odd = head;             // start at 1st node
    struct ListNode* even = head->next;      // start at 2nd node
    struct ListNode* evenHead = even;        // save start of even list

    while (even != NULL && even->next != NULL) {
        odd->next = even->next;   // link odd to the next odd
        odd = odd->next;          // move odd pointer

        even->next = odd->next;   // link even to the next even
        even = even->next;        // move even pointer
    }

    odd->next = evenHead;  // attach even list after odd list
    return head;
    

    return head;
}
