using System;
using Xunit;

namespace two_sum
{
    public class UnitTest
    {
        Solution solution = new Solution();

        [Fact]
        public void Test1()
        {
            Assert.Equal(new int[]{0, 1}, solution.TwoSum(new int[]{2, 7, 11, 15}, 9));
        }
    }
}
