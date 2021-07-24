using System;
using Xunit;

namespace TwoSum.Tests
{
    public class UnitTest
    {
        Solution solution = new Solution();

        [Fact]
        public void Test1()
        {
            Assert.Equal(new int[]{0, 1}, solution.TwoSum(new int[]{2, 7, 11, 15}, 9));
        }

        [Fact]
        public void Test2()
        {
            Assert.Equal(new int[]{1, 2}, solution.TwoSum(new int[]{3, 2, 4}, 6));
        }
    }
}
