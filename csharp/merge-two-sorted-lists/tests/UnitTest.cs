using System;
using Xunit;

namespace MergeTwoSortedLists.Tests
{
    public class UnitTest
    {
        Solution solution = new Solution();

        [Fact]
        public void Test()
        {
            Assert.Equal(
                ListNode.Create(1, 1, 2, 3, 4, 4), 
                solution.MergeTwoLists(ListNode.Create(1, 2, 4), ListNode.Create(1, 3 ,4)));
        }
    }
}
