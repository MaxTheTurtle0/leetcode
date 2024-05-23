/**
 * @typedef {Object} ListNode
 * @property {number} val
 * @property {ListNode} next
 */


/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} n
 * @return {ListNode}
 */
function removeNthFromEnd(head, n) {
    let dummy = new ListNode(0, head);
    let slow = dummy;
    let fast = dummy;

    for (let i = 0; i < n + 1; i++) {
        fast = fast.next;
    }

    while (fast) {
        fast = fast.next;
        slow = slow.next;
    }

    slow.next = slow.next.next;
    
    return dummy.next;
};
