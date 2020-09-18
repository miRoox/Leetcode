using System;
using Xunit;

namespace merge_two_sorted_lists
{
    public class ListUnitTest
    {
        [Fact]
        public void Test0()
        {
            Assert.Null(ListNode.Create());
        }

        [Fact]
        public void Test1()
        {
            Assert.Equal(new ListNode(1), ListNode.Create(1));
        }

        [Fact]
        public void Test2()
        {
            Assert.Equal(new ListNode(1, new ListNode(2)), ListNode.Create(1, 2));
        }
    }
}
