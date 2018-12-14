using System;
using System.Linq;
using System.Threading.Tasks;
using AoC.Abstractions;

namespace AoC.Y2018.Solutions
{
    public class D01P1 : SolutionBase
    {
        public override Task Run()
        {
            var result = InputLines
                .Select(e => e.Trim().TrimStart('+'))
                .Select(e => Convert.ToInt64(e, 10))
                .Aggregate((a, e) => a + e);

            Console.WriteLine(result);
            return Task.CompletedTask;
        }
    }
}
