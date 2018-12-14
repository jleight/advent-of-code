using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using AoC.Abstractions;

namespace AoC.Y2018.Solutions
{
    public class D01P2 : SolutionBase
    {
        public override Task Run()
        {
            var input = InputLines
                .Select(e => e.Trim().TrimStart('+'))
                .Select(e => Convert.ToInt64(e, 10))
                .ToList();

            var seen = new HashSet<long>();

            var index = 0;
            var aggregate = 0L;
            while (seen.Add(aggregate))
            {
                aggregate += input[index];
                index = (index + 1) % input.Count;
            }

            Console.WriteLine(aggregate);
            return Task.CompletedTask;
        }
    }
}
