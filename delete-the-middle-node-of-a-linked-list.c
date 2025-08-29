struct ListNode* deleteMiddle(struct ListNode* head) {
    if (head == NULL) return NULL;
    if (head->next == NULL) {  // only one node
        free(head);
        return NULL;
    }

    // 1) Count nodes
    int counter = 0;
    struct ListNode *curr = head;
    while (curr != NULL) {
        counter++;
        curr = curr->next;
    }

    // 2) Find the middle index
    int mid = counter / 2;

    // 3) Traverse again to reach node before mid
    curr = head;
    for (int i = 0; i < mid - 1; i++) {
        curr = curr->next;
    }

    // 4) Remove the middle node
    struct ListNode *toDelete = curr->next;
    curr->next = toDelete->next; // skip the middle
    free(toDelete);

    // 5) Return the (possibly unchanged) head
    return head;
}
