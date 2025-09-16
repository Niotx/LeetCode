int pairSum(struct ListNode* head) {
    struct ListNode* original = head;
    int count = 0;
    while (head != NULL) {
        count++;
        head = head->next;
    }
    int list[count];
    int index = 0;
    head = original;
    while (head != NULL) {
        list[index] = head->val;
        head = head->next;
        index++;
    }
    int listC[count/2];
    int num = 0;
     for (int i = 0; i < count/2; i++) {
        listC[i] = list[i] + list[count - 1 - i];
    }
    int result = 0;
    for (int i = 0; i < count/2; i++) {
        if (listC[i] > result){
            result = listC[i];
        }
    }
    return result;
}
