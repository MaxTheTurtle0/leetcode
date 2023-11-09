/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
function middleNode(head) {
    let listLength = 0;
    let currentNode = head; 
    while (currentNode.next !== null) {
        listLength++;
        currentNode = currentNode.next;
    }

    for (let i = 0; i < listLength / 2; i++) {
        head = head.next;
    }
    return head;
};

/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
function middleNodeSmart(head) {
    // this is the fast pointer that will move twice as fast as the slow pointer 
    // so when the fast pointer reaches the end of the list, the slow pointer will be in the middle
    let fast = head;
    while (fast !== null && fast.next !== null) {
        fast = fast.next.next;
        head = head.next;
    }
    return head;
};
