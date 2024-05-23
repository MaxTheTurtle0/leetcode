# Definition for singly-linked list.
from typing import Optional


class ListNode:
     def __init__(self, val=0, next=None):
         self.val = val
         self.next = next

def removeNthFromEnd(head: Optional[ListNode], n: int) -> Optional[ListNode]:

    dummy = ListNode(0, head)
    fast = dummy
    slow = dummy
    
    for _ in range(n + 1):
        fast = fast.next

    while fast:
        fast = fast.next
        slow = slow.next

    slow.next = slow.next.next

    return dummy.next
