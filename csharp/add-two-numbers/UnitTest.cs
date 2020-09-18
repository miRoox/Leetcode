using System;
using System.Collections.Generic;
using Xunit;

namespace add_two_numbers
{
    public class UnitTest
    {
        Solution solution = new Solution();

        [Theory]
        [MemberData(nameof(Data))]
        public void Test(ListNode l1, ListNode l2, ListNode sum)
        {
            Assert.Equal(sum, solution.AddTwoNumbers(l1, l2));
        }

        public static IEnumerable<object[]> Data =>
            new List<object[]>
            {
                new object[] {
                    ListNode.Create(2, 4, 3),
                    ListNode.Create(5, 6, 4),
                    ListNode.Create(7, 0, 8),
                },
                new object[] {
                    ListNode.Create(0),
                    ListNode.Create(0),
                    ListNode.Create(0),
                },
                new object[] {
                    ListNode.Create(1, 6, 0, 3, 3, 6, 7, 2, 0, 1),
                    ListNode.Create(6, 3, 0, 8, 9, 6, 6, 9, 6, 1),
                    ListNode.Create(7, 9, 0, 1, 3, 3, 4, 2, 7, 2),
                },
            };
    }
}
